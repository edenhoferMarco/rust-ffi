use std::env;
use std::path::Path;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    let rust_client_root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path_to_lib = Path::new(&rust_client_root).join("../target/release").to_str().unwrap().to_string();
    println!("cargo::rustc-link-search={path_to_lib}/");
    println!("cargo:rustc-link-lib=rust_c_lib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../rust-c-lib/bindings.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(Path::new("./src/bindings.rs"))
        .expect("Couldn't write bindings!");
}