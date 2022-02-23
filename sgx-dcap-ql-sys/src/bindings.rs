use sgx_types::{
    c_char, sgx_ql_path_type_t, sgx_ql_request_policy_t, sgx_quote3_error_t, sgx_report_t,
    sgx_target_info_t, uint32_t, uint8_t,
};

// FROM: https://github.com/apache/incubator-teaclave-sgx-sdk/blob/d107bd0718f723221750a4f2973451b386cbf9d2/sgx_types/src/function.rs#L710
extern "C" {
    //
    // sgx_dcap_ql_wrapper.h
    //
    pub fn sgx_qe_set_enclave_load_policy(policy: sgx_ql_request_policy_t) -> sgx_quote3_error_t;
    pub fn sgx_qe_get_target_info(p_qe_target_info: *mut sgx_target_info_t) -> sgx_quote3_error_t;
    pub fn sgx_qe_get_quote_size(p_quote_size: *mut uint32_t) -> sgx_quote3_error_t;
    pub fn sgx_qe_get_quote(
        p_app_report: *const sgx_report_t,
        quote_size: uint32_t,
        p_quote: *mut uint8_t,
    ) -> sgx_quote3_error_t;
    pub fn sgx_qe_cleanup_by_policy() -> sgx_quote3_error_t;

    /* intel DCAP 1.6 */
    pub fn sgx_ql_set_path(
        path_type: sgx_ql_path_type_t,
        p_path: *const c_char,
    ) -> sgx_quote3_error_t;
}
