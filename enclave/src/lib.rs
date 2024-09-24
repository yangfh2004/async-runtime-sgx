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
#[cfg(target_vendor = "teaclave")]
use exception::ExceptionHandler;
use lazy_static::lazy_static;
use std::time::Duration;
use tokio::runtime::{Builder, Runtime};
use tokio::time::sleep;

lazy_static! {
    static ref RUNTIME: Runtime = Builder::new_multi_thread()
    .worker_threads(8) // TCS = 4 * 2 (extra for dns worker) + 1 = 9. 1 reserved for initializer
    .enable_all()
    .build().unwrap();
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn init_runtime() {
    #[cfg(target_vendor = "teaclave")]
    std::backtrace::enable_backtrace(std::backtrace::PrintFormat::Full)
        .expect("Should not have failed according to documentation");
    let rt = &*RUNTIME.handle();
    rt.block_on(async {
        sleep(Duration::from_millis(10)).await;
        println!("Hello from the enclave!");
    });
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn spawn_http_request(count: u64) {
    let rt = &*RUNTIME.handle();
    let mut handles = Vec::new();
    for i in 0..count {
        let handle = rt.spawn(async move {
            #[cfg(target_vendor = "teaclave")]
            let _handle = ExceptionHandler::new().unwrap();
            let client = reqwest::Client::new();
            let res = {
                sleep(Duration::from_millis(i * 10)).await;
                println!("{} ms delay", i * 10);
                // get some fun facts about cats
                client
                    .get("https://catfact.ninja/fact")
                    .send()
                    .await
                    .unwrap()
            };
            println!("{:?}", res.text().await.unwrap());
        });
        handles.push(handle);
    }
    rt.block_on(async {
        for handle in handles {
            handle.await.unwrap();
        }
    });
}
