// Re-export the `sgx_dcap_ql` symbols from `sgx_types`.
pub use sgx_types::{
    // sgx_dcap_ql_wrapper.h
    sgx_qe_cleanup_by_policy,
    sgx_qe_get_quote,
    sgx_qe_get_quote_size,
    sgx_qe_get_target_info,
    sgx_qe_set_enclave_load_policy,
    // Intel DCAP 1.6
    sgx_ql_set_path,
};
