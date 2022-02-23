use std::ffi::OsStr;

use libloading::os::unix::{Library, Symbol};

use sgx_types::{sgx_quote3_error_t, sgx_report_t, sgx_target_info_t, uint32_t, uint8_t};

pub trait QuotingEnclave {
    unsafe fn get_target_info(&self) -> Result<sgx_target_info_t, libloading::Error>;
    unsafe fn get_quote_size(&self) -> Result<uint32_t, libloading::Error>;
    unsafe fn get_quote(
        &self,
        report: sgx_report_t,
        quote_size: uint32_t,
    ) -> Result<Box<[u8]>, libloading::Error>;
}

#[derive(Debug)]
pub struct DylibQuotingEnclave {
    qe_lib: Library,
}

type QeTargetInfo = unsafe extern "C" fn(*mut sgx_target_info_t) -> sgx_quote3_error_t;
type QeQuoteSize = unsafe extern "C" fn(*mut uint32_t) -> sgx_quote3_error_t;
type QeQuote =
    unsafe extern "C" fn(*const sgx_report_t, uint32_t, *mut uint8_t) -> sgx_quote3_error_t;

impl DylibQuotingEnclave {
    pub unsafe fn new<P: AsRef<OsStr>>(
        filename: P,
    ) -> Result<DylibQuotingEnclave, libloading::Error> {
        Ok(DylibQuotingEnclave {
            qe_lib: unsafe { Library::new(filename) }?,
        })
    }
}

impl QuotingEnclave for DylibQuotingEnclave {
    unsafe fn get_target_info(&self) -> Result<sgx_target_info_t, libloading::Error> {
        let mut target_info = sgx_target_info_t::default();
        let func: Symbol<QeTargetInfo> = unsafe { self.qe_lib.get(b"sgx_qe_get_target_info") }?;
        let qe_result = unsafe { func(&mut target_info as _) };
        match qe_result {
            sgx_quote3_error_t::SGX_QL_SUCCESS => Ok(target_info),
            _ => todo!(),
        }
    }

    unsafe fn get_quote_size(&self) -> Result<uint32_t, libloading::Error> {
        let mut quote_size = uint32_t::default();
        let func: Symbol<QeQuoteSize> = unsafe { self.qe_lib.get(b"sgx_qe_get_quote_size") }?;
        let qe_result = unsafe { func(&mut quote_size as _) };
        match qe_result {
            sgx_quote3_error_t::SGX_QL_SUCCESS => Ok(quote_size),
            _ => todo!(),
        }
    }

    unsafe fn get_quote(
        &self,
        report: sgx_report_t,
        quote_size: uint32_t,
    ) -> Result<Box<[u8]>, libloading::Error> {
        let mut quote_vec: Vec<u8> = vec![0; quote_size as usize];
        let func: Symbol<QeQuote> = unsafe { self.qe_lib.get(b"sgx_qe_get_quote") }?;
        let qe_result = unsafe { func(&report, quote_size, quote_vec.as_mut_ptr() as _) };

        match qe_result {
            sgx_quote3_error_t::SGX_QL_SUCCESS => Ok(quote_vec.into_boxed_slice()),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use sgx_types::{sgx_report_t, sgx_target_info_t};

    use super::DylibQuotingEnclave;
    use super::QuotingEnclave;

    // TODO: Provide this value from the environment, and set using build script
    // TODO: Move the mock library code into this repo
    fn get_mock_lib_path() -> &'static str {
        "/home/longtomjr/Workspace/ntls/dcap_attestation/libmock_dcap_quoteprov.so"
    }

    fn get_quoting_enclave() -> DylibQuotingEnclave {
        // TODO: Use once cell to ensure only one quoting enclave can be loaded for the tests.
        let result = unsafe { DylibQuotingEnclave::new(get_mock_lib_path()) };
        assert!(result.is_ok(), "result = {:?}", result);
        result.unwrap()
    }

    #[test]
    fn get_target_info_works() {
        let quoting_enclave = get_quoting_enclave();

        let result = unsafe { quoting_enclave.get_target_info() };

        assert!(result.is_ok(), "result = {:?}", result);

        let target_info = result.unwrap();

        // TODO: Add consts with the result to the mock quoteprov library, load the library and get the values in the test code.
        assert_ne!(target_info, sgx_target_info_t::default());
    }

    #[test]
    fn get_quote_works() {
        let quoting_enclave = get_quoting_enclave();
        let quote_size_result = unsafe { quoting_enclave.get_quote_size() };
        assert!(
            quote_size_result.is_ok(),
            "quote_size_result = {:?}",
            quote_size_result,
        );

        let quote_size = quote_size_result.unwrap();

        let result = unsafe { quoting_enclave.get_quote(sgx_report_t::default(), quote_size) };

        assert!(result.is_ok(), "result = {:?}", result);
        let quote = result.unwrap();

        // TODO: Add consts to mock library and assert that the value is the same.
        println!("{:?}", quote);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
