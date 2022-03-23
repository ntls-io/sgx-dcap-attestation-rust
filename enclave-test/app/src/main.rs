extern crate sgx_types;
extern crate sgx_urts;

#[path = "../codegen/Enclave_u.rs"]
mod enclave_u;

use dcap_attestation;
use enclave_u::ecall_test;
use sgx_types::{
    sgx_attributes_t,
    sgx_launch_token_t,
    sgx_misc_attribute_t,
    sgx_qe_get_target_info,
    sgx_report_t,
    sgx_status_t,
    sgx_target_info_t,
    SgxResult,
};
use sgx_urts::SgxEnclave;

static ENCLAVE_FILE: &str = "enclave.signed.so";

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )
}

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        }
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        }
    };

    // let qe_target_info = sgx_target_info_t::default();
    let qe_target_info = unsafe { dcap_attestation::get_target_info() }.unwrap();

    let mut report = sgx_report_t::default();
    let mut retval = sgx_status_t::SGX_SUCCESS;

    let result = unsafe {
        enclave_u::enclave_create_report(
            enclave.geteid(),
            &mut retval,
            &qe_target_info,
            &mut report,
        )
    };
    println!("Enclave Report: {:?}", report);

    let quote_size = unsafe { dcap_attestation::get_quote_size() }.unwrap();
    println!("Quote Size: {:?}", quote_size);

    let quote = unsafe { dcap_attestation::get_quote(report, quote_size) }.unwrap();
    println!("Quote: {:?}", quote);

    println!("{:?}", report);

    match result {
        sgx_status_t::SGX_SUCCESS => {}
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    println!("[+] ecall_test success...");

    enclave.destroy();
}
