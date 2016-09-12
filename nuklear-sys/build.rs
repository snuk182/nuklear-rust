extern crate gcc;

fn main() {
    /*gcc::Config::new()
	    .file("nuklear-c/nuklear/nuklear.h")
	    .define("NK_IMPLEMENTATION", None)
        .define("NK_INCLUDE_FIXED_TYPES", None)
        .define("NK_INCLUDE_COMMAND_USERDATA", None)
        .define("NK_INCLUDE_VERTEX_BUFFER_OUTPUT", None)
        .define("NK_INCLUDE_FONT_BAKING", None)
        .compile("libnuklear.a");*/
    gcc::compile_library("libnuklear.a", &["nuklear-c/bind.c"]);
}