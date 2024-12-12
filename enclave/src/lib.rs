// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..
#[cfg(target_vendor = "teaclave")]
extern crate sgx_libc;
#[cfg(target_vendor = "teaclave")]
extern crate sgx_oc;
#[cfg(target_vendor = "teaclave")]
extern crate sgx_trts;
#[cfg(target_vendor = "teaclave")]
extern crate sgx_types;

#[cfg(target_vendor = "teaclave")]
mod exception;
use anyhow::{Context, Result};
use bytes::Bytes;
#[cfg(target_vendor = "teaclave")]
use exception::ExceptionHandler;
use http_body_util::{BodyExt, Empty};
use lazy_static::lazy_static;
use sgx_types::error::SgxStatus;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::time::sleep;

use hyper_rustls::HttpsConnector;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use rustls::RootCertStore;
use serde::Deserialize;

lazy_static! {
    static ref RUNTIME: Runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn init_runtime() -> SgxStatus {
    #[cfg(target_vendor = "teaclave")]
    std::backtrace::enable_backtrace(std::backtrace::PrintFormat::Full)
        .expect("Should not have failed according to documentation");
    let rt = &*RUNTIME.handle();
    rt.block_on(async {
        sleep(Duration::from_millis(10)).await;
        println!("Tokio runtime initialized!");
    });
    SgxStatus::Success
}

#[derive(Debug, Deserialize, Clone)]
pub struct TickerPrice {
    pub amount: String,
    pub base: String,
    pub currency: String,
}

pub const COINBASE_TICKER_URL: &str = "https://api.coinbase.com/v2/prices/ETH-USD/spot";

/// This version uses the `hyper_rustls` crate and the legacy `hyper` Client, which handles client management.
/// I tested this only to investigate what aspect of the client might be incompatible with the enclave,
/// but I wasn’t able to reproduce the error. I do NOT recommend using this approach, as I prefer the lower-level approach.
pub async fn fetch_url() -> Result<()> {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(config)
        .https_or_http()
        .enable_http1()
        .build();
    // NOTE: This client is legacy, and I don't believe that we need it. See: https://hyper.rs/guides/1/client/connectors/
    // I just wanted to check whether or not it worked for my own edification.
    let client: Client<HttpsConnector<HttpConnector>, Empty<Bytes>> =
        Client::builder(TokioExecutor::new()).build(https);
    let url = COINBASE_TICKER_URL.parse::<hyper::Uri>().unwrap();
    let res = client.get(url).await?;
    let bytes = res.into_body().collect().await?.to_bytes();
    let body = String::from_utf8(bytes.to_vec()).context("Response body is not valid UTF-8")?;
    // deserialize the response into a TickerPrice struct like this:
    // {"data":{"amount":"96663.235","base":"BTC","currency":"USD"}}
    let data: serde_json::Value =
        serde_json::from_str(&body).context("Failed to parse JSON response")?;
    let ticker_price: TickerPrice =
        serde_json::from_str(&data["data"].to_string()).context("Failed to parse JSON response")?;
    println!("ETH MARKETS: {:?}", ticker_price);
    Ok(())
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn spawn_http_request(count: u64) -> SgxStatus {
    #[cfg(target_vendor = "teaclave")]
    let _handle = ExceptionHandler::new().unwrap();
    let rt = &*RUNTIME.handle();
    let mut handles = Vec::new();
    for _ in 0..count {
        let handle = rt.spawn(fetch_url());
        handles.push(handle);
    }
    for handle in handles {
        rt.block_on(handle).unwrap().unwrap();
    }
    SgxStatus::Success
}
