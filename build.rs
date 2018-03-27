extern crate cc;

use std::env;
use std::process::Command;

fn main() {
    let mut builder = cc::Build::new();

    for flag in llvm_config("--cflags").trim().split(" ") {
        builder.flag_if_supported(flag);
    }

    builder
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-fno-rtti")
        .file("llvm/internalize.cpp")
        .file("llvm/helpers/visitors.cpp")
        .compile("librust-ptx-llvm-stuff.a");
}

fn llvm_config(arg: &str) -> String {
    let executable_path =
        env::var("DEP_LLVM_CONFIG_PATH").expect("Unable to find env variable DEP_LLVM_CONFIG_PATH");

    let output = Command::new(executable_path)
        .arg(arg)
        .output()
        .expect("Unable to execute llvm-config");

    String::from_utf8(output.stdout).expect("Output from llvm-config was not valid UTF-8")
}
