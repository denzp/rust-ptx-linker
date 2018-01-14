extern crate cc;

use std::env;
use std::path::PathBuf;

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

    link_rustc_llvm_lib();
}

fn link_rustc_llvm_lib() {
    match find_rustc_llvm_lib() {
        Some(path) => {
            let location = path.as_path().parent().unwrap().to_str().unwrap();
            let name = path.as_path().file_stem().unwrap().to_str().unwrap();

            let libname = match name.starts_with("lib") {
                true => &name[3..],
                false => name,
            };

            println!("cargo:rustc-link-search=native={}", location);
            println!("cargo:rustc-link-lib=dylib={}", libname);
        }

        None => {
            unreachable!("Unable to get location of 'librustc_llvm'.");
        }
    }
}

fn find_rustc_llvm_lib() -> Option<PathBuf> {
    let rustup_home = env::var("RUSTUP_HOME")
        .expect("Unable to get 'RUSTUP_HOME' env variable. Do you have rustup installed?");

    let rustup_toolchain = env::var("RUSTUP_TOOLCHAIN")
        .expect("Unable to get 'RUSTUP_TOOLCHAIN' env variable. Do you have rustup installed?");

    let rustc_libs_path = PathBuf::from(format!(
        "{root}/toolchains/{toolchain}/lib/",
        root = rustup_home,
        toolchain = rustup_toolchain
    ));

    if let Ok(entries) = rustc_libs_path.read_dir() {
        let mut matching_entries = entries.filter(|entry| {
            entry
                .as_ref()
                .unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("librustc_llvm")
        });

        if let Some(entry) = matching_entries.nth(0) {
            return Some(entry.unwrap().path());
        }
    }

    None
}
