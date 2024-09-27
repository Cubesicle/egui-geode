use std::env;
use std::path::PathBuf;

fn main() {
    //println!("cargo:rustc-link-search={}/build", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-lib={}/build/cubesicle.test.dll", env::var("CARGO_MANIFEST_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("src/ffi.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}