extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .opt_level(0)
        .include("llvm/headers")
        .flag_if_supported("-pedantic")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-fno-rtti")
        .flag_if_supported("-Wno-unused-parameter")
        .define("_GLIBCXX_USE_CXX11_ABI", "0")
        .file("llvm/find-external-refs.cpp")
        .file("llvm/internalize.cpp")
        .file("llvm/rename-globals.cpp")
        .file("llvm/helpers/visitors.cpp")
        .compile("librust-ptx-llvm-stuff.a");
}
