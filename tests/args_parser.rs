extern crate ptx_linker;

use ptx_linker::session::*;
use std::path::PathBuf;

#[test]
fn it_should_parse_args() {
    let args = &[
        "-L",
        "/rustlib/lib",
        "/kernel/target/debug/deps/kernel.0.o",
        "/kernel/target/debug/deps/kernel.crate.metadata.o",
        "-o",
        "/kernel/target/debug/deps/libkernel.ptx",
        "-L",
        "/kernel/target/debug/deps",
        "-L",
        "/kernel/target/debug/deps",
        "-L",
        "~/rustlib/nvptx64-nvidia-cuda/lib",
        "-Bstatic",
        "--whole-archive",
        "/tmp/rustc.Ew934MzC8cj0/liblib-f0faab0dbaa9f7ef.rlib",
        "--no-whole-archive",
        "--whole-archive",
        "/tmp/rustc.Ew934MzC8cj0/libother-6b4931ba2f43f84b.rlib",
        "--no-whole-archive",
    ];

    let current_session = ArgsParser::new(prepare(args)).create_session().unwrap();

    let ref_session = Session {
        emit: vec![Output::PTXAssembly],
        configuration: Configuration::Debug,

        output: Some(PathBuf::from("/kernel/target/debug/deps/libkernel.ptx")),

        include_rlibs: vec![
            PathBuf::from("/tmp/rustc.Ew934MzC8cj0/liblib-f0faab0dbaa9f7ef.rlib"),
            PathBuf::from("/tmp/rustc.Ew934MzC8cj0/libother-6b4931ba2f43f84b.rlib"),
        ],

        include_bitcode_modules: vec![PathBuf::from("/kernel/target/debug/deps/kernel.0.o")],
    };

    assert_eq!(current_session, ref_session);
}

#[test]
fn it_should_parse_optimization() {
    let args = &["-o", "/kernel/target/debug/deps/libkernel.ptx", "-O1"];

    let current_session = ArgsParser::new(prepare(args)).create_session().unwrap();

    let ref_session = Session {
        emit: vec![Output::PTXAssembly],
        configuration: Configuration::Release,

        output: Some(PathBuf::from("/kernel/target/debug/deps/libkernel.ptx")),

        include_rlibs: vec![],
        include_bitcode_modules: vec![],
    };

    assert_eq!(current_session, ref_session);
}

fn prepare(args: &[&str]) -> Vec<String> {
    args.iter().map(|item| String::from(*item)).collect()
}
