#![no_std]

extern crate sgx_types;
#[macro_use]
extern crate sgx_tstd as std;

use std::io::{self, Write};
use std::slice;

use sgx_tse::rsgx_create_report;
use sgx_types::{sgx_report_data_t, sgx_report_t, sgx_status_t, sgx_target_info_t};

#[no_mangle]
pub extern "C" fn ecall_test(some_string: *const u8, some_len: usize) -> sgx_status_t {
    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    println!("Message from the enclave");

    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn enclave_create_report(
    p_qe3_target: *const sgx_target_info_t,
    p_report: *mut sgx_report_t,
) -> sgx_status_t {
    let mut p_data = sgx_report_data_t::default();
    p_data.d[0..32].copy_from_slice(&[8; 32]);

    let report = rsgx_create_report(&*p_qe3_target, &p_data).unwrap();
    *p_report = report;
    sgx_status_t::SGX_SUCCESS
}