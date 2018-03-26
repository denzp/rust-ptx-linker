use std::path::PathBuf;
use tempdir::TempDir;

use ptx_linker::linker::*;
use ptx_linker::session::{Configuration, Output, Session};

#[test]
fn it_should_emit_correct_debug_ir() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let actual_output = directory.path().join("module-name.ll");
    let reference_output = PathBuf::from("tests/codegen/dependencies/outputs/debug.ll");

    session.emit = vec![Output::IntermediateRepresentation];
    session.output = Some(actual_output.clone());
    session.configuration = Configuration::Debug;
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.9elsx31vb4it187.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.3912kojcc92n58xs.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-8d1ba91f02f92ba6.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-890580f027b43b6d.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_files_eq!(actual_output, reference_output);
}

#[test]
fn it_should_emit_correct_release_ir() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let actual_output = directory.path().join("module-name.ll");
    let reference_output = PathBuf::from("tests/codegen/dependencies/outputs/release.ll");

    session.emit = vec![Output::IntermediateRepresentation];
    session.output = Some(actual_output.clone());
    session.configuration = Configuration::Release;
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/release.example.example0-4b80eefc6308f6a9a893a7e7f84d14d8.rs.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/release.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-8d1ba91f02f92ba6.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-890580f027b43b6d.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_files_eq!(actual_output, reference_output);
}

#[test]
fn it_should_emit_bc() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let actual_output = directory.path().join("module-name.bc");

    session.emit = vec![Output::Bitcode];
    session.output = Some(actual_output.clone());
    session.configuration = Configuration::Release;
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/release.example.example0-4b80eefc6308f6a9a893a7e7f84d14d8.rs.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/release.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-8d1ba91f02f92ba6.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-890580f027b43b6d.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
}

#[test]
fn it_should_emit_correct_debug_asm() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let actual_output = directory.path().join("module-name.ptx");
    let reference_output = PathBuf::from("tests/codegen/dependencies/outputs/debug.ptx");

    session.emit = vec![Output::PTXAssembly];
    session.output = Some(actual_output.clone());
    session.configuration = Configuration::Debug;
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.9elsx31vb4it187.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.3912kojcc92n58xs.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-8d1ba91f02f92ba6.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-890580f027b43b6d.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libcore-4302314abb088ec8.rlib",
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
    let reference_output = PathBuf::from("tests/codegen/dependencies/outputs/release.ptx");

    session.emit = vec![Output::PTXAssembly];
    session.output = Some(actual_output.clone());
    session.configuration = Configuration::Release;
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/release.example.example0-4b80eefc6308f6a9a893a7e7f84d14d8.rs.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/release.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-8d1ba91f02f92ba6.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-890580f027b43b6d.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_files_eq!(actual_output, reference_output);
}
