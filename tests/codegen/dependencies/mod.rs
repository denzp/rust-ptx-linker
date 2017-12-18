use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, Read};
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
        "tests/codegen/dependencies/inputs/debug.example.example0.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.example1.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-74eeab9afcfbdc2e.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-efe1e3357e6f3320.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_files_equal(actual_output, reference_output);
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
        "tests/codegen/dependencies/inputs/release.example.example0.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/release.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-74eeab9afcfbdc2e.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-efe1e3357e6f3320.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_files_equal(actual_output, reference_output);
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
        "tests/codegen/dependencies/inputs/release.example.example0.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/release.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-74eeab9afcfbdc2e.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-efe1e3357e6f3320.rlib",
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
        "tests/codegen/dependencies/inputs/debug.example.example0.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.example1.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/debug.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-74eeab9afcfbdc2e.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-efe1e3357e6f3320.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_files_equal(actual_output, reference_output);
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
        "tests/codegen/dependencies/inputs/release.example.example0.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/dependencies/inputs/release.example.crate.metadata.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_math-74eeab9afcfbdc2e.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libdummy_utils-efe1e3357e6f3320.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/dependencies/inputs/libcore-4302314abb088ec8.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_files_equal(actual_output, reference_output);
}

fn assert_files_equal(actual_file_path: PathBuf, ref_file_path: PathBuf) {
    let mut actual_file = BufReader::new(File::open(actual_file_path).unwrap());
    let mut ref_file = BufReader::new(File::open(ref_file_path).unwrap());

    let mut actual_contents = String::new();
    let mut ref_contents = String::new();

    actual_file.read_to_string(&mut actual_contents).unwrap();
    ref_file.read_to_string(&mut ref_contents).unwrap();

    assert_diff!(&ref_contents, &actual_contents, "\n", 0);
}
