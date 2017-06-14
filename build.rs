extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .opt_level(0)
        .include("llvm/headers")
        .flag("-pedantic")
        .flag("-Wall")
        .flag("-Werror")
        .flag("-std=c++11")
        .flag("-D_GLIBCXX_USE_CXX11_ABI=0")
        .file("llvm/external-refs.cpp")
        .compile("librust-ptx-llvm-stuff.a");
}

