#![deny(clippy::mem_forget)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

#[cfg(feature = "test")]
use sgx_dcap_ql_sys::MockSgxDcapQl as SgxDcapQlSys;
use sgx_dcap_ql_sys::SgxDcapQl;
#[cfg(not(feature = "test"))]
use sgx_dcap_ql_sys::SgxDcapQlSys;
use sgx_types::{sgx_quote3_error_t, sgx_report_t, sgx_target_info_t, uint32_t};

pub unsafe fn get_target_info() -> Result<sgx_target_info_t, sgx_quote3_error_t> {
    let mut target_info = sgx_target_info_t::default();
    let qe_result = unsafe { SgxDcapQlSys::sgx_qe_get_target_info(&mut target_info) };
    match qe_result {
        sgx_quote3_error_t::SGX_QL_SUCCESS => Ok(target_info),
        _ => Err(qe_result),
    }
}

pub unsafe fn get_quote_size() -> Result<uint32_t, sgx_quote3_error_t> {
    let mut quote_size = uint32_t::default();
    let qe_result = unsafe { SgxDcapQlSys::sgx_qe_get_quote_size(&mut quote_size) };
    match qe_result {
        sgx_quote3_error_t::SGX_QL_SUCCESS => Ok(quote_size),
        _ => Err(qe_result),
    }
}

pub unsafe fn get_quote(
    report: sgx_report_t,
    quote_size: uint32_t,
) -> Result<Box<[u8]>, sgx_quote3_error_t> {
    let mut quote_vec: Vec<u8> = vec![0; quote_size as usize];
    let qe_result =
        unsafe { SgxDcapQlSys::sgx_qe_get_quote(&report, quote_size, quote_vec.as_mut_ptr()) };

    match qe_result {
        sgx_quote3_error_t::SGX_QL_SUCCESS => Ok(quote_vec.into_boxed_slice()),
        _ => Err(qe_result),
    }
}

#[cfg(test)]
mod test {
    use core::slice;
    use std::sync::{Mutex, MutexGuard};

    use once_cell::sync::Lazy;
    use sgx_types::sgx_report_t;

    use super::*;

    // Using a mutex to synchronize tests using that use static mock contexts
    // since the expectations are global for static methods in mockall.
    // see: https://docs.rs/mockall/latest/mockall/index.html#static-methods
    static MTX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    fn get_lock(m: &'static Mutex<()>) -> MutexGuard<'static, ()> {
        match m.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    #[test]
    fn get_target_info_success() {
        let _m = get_lock(&MTX);
        let ctx = SgxDcapQlSys::sgx_qe_get_target_info_context();
        ctx.expect()
            .once()
            .returning(|target_info: *mut sgx_target_info_t| {
                unsafe {
                    (*target_info).reserved1 = [1u8, 2u8];
                }
                sgx_quote3_error_t::SGX_QL_SUCCESS
            });

        let result = unsafe { get_target_info() };

        assert!(result.is_ok());
        assert_ne!(result.unwrap(), sgx_target_info_t::default());
    }

    #[test]
    fn get_target_info_error() {
        let _m = get_lock(&MTX);
        let err = sgx_quote3_error_t::SGX_QL_ERROR_OUT_OF_MEMORY;
        let ctx = SgxDcapQlSys::sgx_qe_get_target_info_context();
        ctx.expect()
            .once()
            .returning(move |target_info: *mut sgx_target_info_t| err.clone());

        let result = unsafe { get_target_info() };

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), err)
    }

    #[test]
    fn get_quote_size_success() {
        let _m = get_lock(&MTX);
        let expected_quote_size = 5;
        let ctx = SgxDcapQlSys::sgx_qe_get_quote_size_context();
        ctx.expect().once().returning(move |quote_size: *mut u32| {
            unsafe { *quote_size = expected_quote_size }
            sgx_quote3_error_t::SGX_QL_SUCCESS
        });

        let result = unsafe { get_quote_size() };

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_quote_size);
    }

    #[test]
    fn get_quote_size_error() {
        let _m = get_lock(&MTX);
        let err = sgx_quote3_error_t::SGX_QL_ERROR_OUT_OF_MEMORY;
        let ctx = SgxDcapQlSys::sgx_qe_get_quote_size_context();
        ctx.expect()
            .once()
            .returning(move |quote_size: *mut u32| err.clone());

        let result = unsafe { get_quote_size() };

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), err);
    }

    #[test]
    fn get_quote_success() {
        let _m = get_lock(&MTX);
        let quote_size = 5u32;
        let expected_quote = vec![2u8; quote_size as usize];
        let expected_quote_clone = expected_quote.clone();
        let report = sgx_report_t::default();

        let ctx = SgxDcapQlSys::sgx_qe_get_quote_context();
        ctx.expect().once().returning(
            move |report: *const sgx_report_t, quote_size: u32, quote_ptr: *mut u8| {
                let mut quote =
                    unsafe { slice::from_raw_parts_mut(quote_ptr, quote_size as usize) };
                quote.copy_from_slice(&expected_quote_clone);
                sgx_quote3_error_t::SGX_QL_SUCCESS
            },
        );

        let result = unsafe { get_quote(report, quote_size) };

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_quote.into_boxed_slice());
    }

    #[test]
    fn get_quote_error() {
        let _m = get_lock(&MTX);
        let quote_size = 5u32;
        let report = sgx_report_t::default();
        let err = sgx_quote3_error_t::SGX_QL_ERROR_OUT_OF_MEMORY;

        let ctx = SgxDcapQlSys::sgx_qe_get_quote_context();

        ctx.expect().once().returning(
            move |report: *const sgx_report_t, quote_size: u32, quote_ptr: *mut u8| err.clone(),
        );

        let result = unsafe { get_quote(report, quote_size) };

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), err);
    }
}
