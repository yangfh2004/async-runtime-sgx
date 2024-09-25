extern crate sgx_types;
extern crate sgx_urts;

use sgx_types::error::SgxStatus;
use sgx_types::types::*;
use sgx_urts::enclave::SgxEnclave;

static ENCLAVE_FILE: &str = "enclave.signed.so";

extern "C" {
    fn spawn_http_request(eid: EnclaveId, retval: *mut SgxStatus, count: u64) -> SgxStatus;
    fn init_runtime(eid: EnclaveId, retval: *mut SgxStatus) -> SgxStatus;
}

fn main() {
    let enclave = SgxEnclave::create(ENCLAVE_FILE, true).unwrap();
    let eid = enclave.eid();
    let mut status: SgxStatus = SgxStatus::Success;
    let ret = unsafe { init_runtime(eid, &mut status) };
    assert_eq!(ret, SgxStatus::Success);
    assert_eq!(status, SgxStatus::Success);
    // measure the time it takes to do 1 http request
    let start = std::time::Instant::now();
    // create http request
    let ret = unsafe { spawn_http_request(eid, &mut status, 1) };
    assert_eq!(ret, SgxStatus::Success);
    assert_eq!(status, SgxStatus::Success);
    let duration = start.elapsed();
    println!("Time taken: {:?} for 1 request", duration);

    // measure the time it takes to do 8 http request
    let start = std::time::Instant::now();
    // create http request
    let count = 8;
    let ret = unsafe { spawn_http_request(eid, &mut status, count) };
    assert_eq!(ret, SgxStatus::Success);
    assert_eq!(status, SgxStatus::Success);

    let duration = start.elapsed();
    println!("Time taken: {:?} for {} requests", duration, count);
}
