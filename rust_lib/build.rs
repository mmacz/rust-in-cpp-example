extern crate cbindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let root_dir = PathBuf::from(&crate_dir)
        .parent()
        .unwrap()
        .display()
        .to_string();
    let output_file = PathBuf::from(&root_dir)
        .join(format!("cbindgen_headers/{}.h", package_name))
        .display()
        .to_string();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::Cxx;

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&output_file);
}
