#[macro_use]
extern crate clap;
extern crate ptx_linker;

use clap::{App, ArgMatches};
use ptx_linker::session::*;
use std::path::PathBuf;

#[test]
fn it_should_parse_args() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches_from_safe(vec![
        "ptx-linker",
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
    ]);

    let expected_session = Session {
        emit: vec![Output::PTXAssembly, Output::IntermediateRepresentation],
        configuration: Configuration::Debug,

        output: Some(PathBuf::from("/kernel/target/debug/deps/libkernel.ptx")),

        include_rlibs: vec![
            PathBuf::from("/tmp/rustc.Ew934MzC8cj0/liblib-f0faab0dbaa9f7ef.rlib"),
            PathBuf::from("/tmp/rustc.Ew934MzC8cj0/libother-6b4931ba2f43f84b.rlib"),
        ],

        include_bitcode_modules: vec![PathBuf::from("/kernel/target/debug/deps/kernel.0.o")],
    };

    assert_eq!(
        CommandLineRequest::from(matches.expect("Unable to parse CLI arguments")),
        CommandLineRequest::Link(expected_session)
    );
}

#[test]
fn it_should_parse_optimization() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches_from_safe(vec![
        "ptx-linker",
        "-o",
        "/kernel/target/debug/deps/libkernel.ptx",
        "-O1",
    ]);

    let expected_session = Session {
        emit: vec![Output::PTXAssembly, Output::IntermediateRepresentation],
        configuration: Configuration::Release,

        output: Some(PathBuf::from("/kernel/target/debug/deps/libkernel.ptx")),

        include_rlibs: vec![],
        include_bitcode_modules: vec![],
    };

    assert_eq!(
        CommandLineRequest::from(matches.expect("Unable to parse CLI arguments")),
        CommandLineRequest::Link(expected_session)
    );
}

#[test]
fn it_should_not_print_unknown_target_json() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches_from_safe(vec![
        "ptx-linker",
        "print",
        "another-target-triple",
    ]);

    assert!(matches.is_err());
}

#[test]
fn it_should_print_target_json() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches_from_safe(vec![
        "ptx-linker",
        "print",
        "nvptx64-nvidia-cuda",
    ]);

    assert_eq!(
        CommandLineRequest::from(matches.expect("Unable to parse CLI arguments")),
        CommandLineRequest::PrintTargetJson
    );
}
