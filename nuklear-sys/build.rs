extern crate gcc;

fn main() {
    gcc::compile_library("libnuklear.a", &["nuklear-c/bind.c"]);
}
