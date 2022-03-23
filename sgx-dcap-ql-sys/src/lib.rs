#[cfg(feature = "bindings")]
mod bindings;
#[cfg(feature = "mock")]
use mockall::automock;
use sgx_types::{
    c_char, sgx_ql_path_type_t, sgx_ql_request_policy_t, sgx_quote3_error_t, sgx_report_t,
    sgx_target_info_t, uint32_t, uint8_t,
};

#[cfg_attr(feature = "mock", automock)]
pub trait SgxDcapQl {
    unsafe fn sgx_qe_set_enclave_load_policy(policy: sgx_ql_request_policy_t)
        -> sgx_quote3_error_t;
    unsafe fn sgx_qe_get_target_info(
        p_qe_target_info: *mut sgx_target_info_t,
    ) -> sgx_quote3_error_t;
    unsafe fn sgx_qe_get_quote_size(p_quote_size: *mut uint32_t) -> sgx_quote3_error_t;
    unsafe fn sgx_qe_get_quote(
        p_app_report: *const sgx_report_t,
        quote_size: uint32_t,
        p_quote: *mut uint8_t,
    ) -> sgx_quote3_error_t;
    unsafe fn sgx_qe_cleanup_by_policy() -> sgx_quote3_error_t;

    /* intel DCAP 1.6 */
    unsafe fn sgx_ql_set_path(
        path_type: sgx_ql_path_type_t,
        p_path: *const c_char,
    ) -> sgx_quote3_error_t;
}

#[cfg(feature = "bindings")]
pub struct SgxDcapQlSys;

#[cfg(feature = "bindings")]
impl SgxDcapQl for SgxDcapQlSys {
    unsafe fn sgx_qe_set_enclave_load_policy(
        policy: sgx_ql_request_policy_t,
    ) -> sgx_quote3_error_t {
        bindings::sgx_qe_set_enclave_load_policy(policy)
    }

    unsafe fn sgx_qe_get_target_info(
        p_qe_target_info: *mut sgx_target_info_t,
    ) -> sgx_quote3_error_t {
        bindings::sgx_qe_get_target_info(p_qe_target_info)
    }

    unsafe fn sgx_qe_get_quote_size(p_quote_size: *mut uint32_t) -> sgx_quote3_error_t {
        bindings::sgx_qe_get_quote_size(p_quote_size)
    }

    unsafe fn sgx_qe_get_quote(
        p_app_report: *const sgx_report_t,
        quote_size: uint32_t,
        p_quote: *mut uint8_t,
    ) -> sgx_quote3_error_t {
        bindings::sgx_qe_get_quote(p_app_report, quote_size, p_quote)
    }

    unsafe fn sgx_qe_cleanup_by_policy() -> sgx_quote3_error_t {
        bindings::sgx_qe_cleanup_by_policy()
    }

    unsafe fn sgx_ql_set_path(
        path_type: sgx_ql_path_type_t,
        p_path: *const c_char,
    ) -> sgx_quote3_error_t {
        bindings::sgx_ql_set_path(path_type, p_path)
    }
}
