use std::env;

fn main() {
    let bindings: bool = !env::var("CARGO_FEATURE_BINDINGS").map_or(false, |val| val == "true");

    if bindings {
        println!("cargo:rustc-link-lib=sgx_dcap_ql");
    }
}
