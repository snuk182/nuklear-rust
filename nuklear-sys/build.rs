extern crate bindgen;
extern crate cc;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};

fn main() {
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Unable to get OUT_DIR"));

    // Tell cargo to invalidate the built crate whenever any of the local sources change
    println!("cargo:rerun-if-changed=nuklear_include.h");
    println!("cargo:rerun-if-changed=wrapper.c");
    println!("cargo:rerun-if-changed=nuklear-c/nuklear/nuklear.h");

    cc::Build::new()
        .debug(true)
        .opt_level(0)
        .warnings(true)
        .file("wrapper.c")
        .try_compile("nuklear")
        .expect("Unable to compile Nuklear library");

    // Tell cargo to tell rustc to statically link the libnuklear library.
    println!("cargo:rustc-link-lib=static=nuklear");

    // Use bindgen to generate unsafe rust bindings to the library
    let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .derive_default(true)
        // Generate bindings for the wrapper
        .header("nuklear_include.h")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .parse_callbacks(Box::new(MacroCallback { macros: Arc::new(RwLock::new(HashSet::new())) }))
        .generate()
        .expect("Unable to generate bindings");

    let dest_path = out_path.join("bindings.rs");
    bindings.write_to_file(dest_path).expect("Couldn't write bindings!");
}

#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        if name == "FP_INFINITE"
            || name == "FP_ZERO"
            || name == "FP_NAN"
            || name == "FP_SUBNORMAL"
            || name == "FP_NORMAL"
        {
            return MacroParsingBehavior::Ignore
        }

        MacroParsingBehavior::Default
    }
}
