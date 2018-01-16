extern crate cc;

use std::path::Path;
use std::process::Command;

fn main() {
    if !Path::new("nuklear-c/nuklear/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status()
            .unwrap();
    }
    cc::Build::new()
        .file("nuklear-c/bind.c")
        .compile("libnuklear");
    //gcc::compile_library("libnuklear.a", &["nuklear-c/bind.c"]);
}
