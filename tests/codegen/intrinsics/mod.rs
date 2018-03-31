use std::path::PathBuf;
use tempdir::TempDir;

use ptx_linker::linker::*;
use ptx_linker::session::{Configuration, Output, Session};

#[test]
fn it_should_emit_correct_release_ir() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let actual_output = directory.path().join("module-name.ll");
    let reference_output = PathBuf::from("tests/codegen/intrinsics/outputs/release.ll");

    session.emit = vec![Output::IntermediateRepresentation];
    session.output = Some(actual_output.clone());
    session.configuration = Configuration::Release;
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/intrinsics/inputs/example.example0.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/intrinsics/inputs/example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/intrinsics/inputs/libnvptx_builtins-165a9d1a4a13bfa2.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/intrinsics/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_files_eq!(actual_output, reference_output);
}

#[test]
fn it_should_emit_correct_release_asm() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let actual_output = directory.path().join("module-name.ptx");
    let reference_output = PathBuf::from("tests/codegen/intrinsics/outputs/release.ptx");

    session.emit = vec![Output::PTXAssembly];
    session.output = Some(actual_output.clone());
    session.configuration = Configuration::Release;
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/intrinsics/inputs/example.example0.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/intrinsics/inputs/example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/intrinsics/inputs/libnvptx_builtins-165a9d1a4a13bfa2.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/intrinsics/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_files_eq!(actual_output, reference_output);
}
