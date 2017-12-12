extern crate gcc;

fn main() {
    gcc::Build::new()
        .cpp(true)
        .opt_level(0)
        .include("llvm/headers")
        .flag("-pedantic")
        .flag("-Wall")
        .flag("-Werror")
        .flag("-std=c++11")
        .flag("-fno-rtti")
        .flag("-Wno-unused-parameter")
        .flag("-D_GLIBCXX_USE_CXX11_ABI=0")
        .file("llvm/find-external-refs.cpp")
        .file("llvm/internalize.cpp")
        .file("llvm/helpers/visitors.cpp")
        .compile("librust-ptx-llvm-stuff.a");
}
