extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Unable to get OUT_DIR"));
    let dest_path = out_path.join("bindings.rs");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    // Tell cargo to invalidate the built crate whenever the single file containing the entire library changes
    println!("cargo:rerun-if-changed=wrapper.c");

    cc::Build::new().warnings(true).file("wrapper.c").try_compile("nuklear").expect("Unable to compile Nuklear library");

    // Tell cargo to tell rustc to statically link the libnuklear library.
    println!("cargo:rustc-link-lib=static=nuklear");

    // Use bindgen to generate unsafe rust bindings to the library
    let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .derive_default(true)
        // Generate bindings for the wrapper
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file(dest_path).expect("Couldn't write bindings!");
}
