#[macro_use]
extern crate difference;
extern crate tempdir;
extern crate ptx_linker;

use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, BufReader};
use tempdir::TempDir;

use ptx_linker::linker::*;
use ptx_linker::session::{Session, Output, Configuration};

#[test]
fn it_should_emit_correct_debug_ir() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let expected_output = directory.path().join("module-name.ll");
    let reference_output = PathBuf::from("tests/codegen/outputs/debug.ll");

    session.emit = vec![Output::IntermediateRepresentation];
    session.output = Some(expected_output.clone());
    session.configuration = Configuration::Debug;
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.0.o"));
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.crate.metadata.o"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_math-5d32b2be875cc4d4.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_utils-315daf14970b3da5.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libcore-c8f041115f42fd27.rlib"));

    assert_eq!(expected_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(expected_output.exists(), true);
    assert_files_equal(expected_output, reference_output);
}

#[test]
fn it_should_emit_correct_release_ir() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let expected_output = directory.path().join("module-name.ll");
    let reference_output = PathBuf::from("tests/codegen/outputs/release.ll");

    session.emit = vec![Output::IntermediateRepresentation];
    session.output = Some(expected_output.clone());
    session.configuration = Configuration::Release;
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.0.o"));
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.crate.metadata.o"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_math-5d32b2be875cc4d4.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_utils-315daf14970b3da5.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libcore-c8f041115f42fd27.rlib"));

    assert_eq!(expected_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(expected_output.exists(), true);
    assert_files_equal(expected_output, reference_output);
}

#[test]
fn it_should_emit_bc() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let expected_output = directory.path().join("module-name.bc");

    session.emit = vec![Output::Bitcode];
    session.output = Some(expected_output.clone());
    session.configuration = Configuration::Release;
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.0.o"));
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.crate.metadata.o"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_math-5d32b2be875cc4d4.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_utils-315daf14970b3da5.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libcore-c8f041115f42fd27.rlib"));

    assert_eq!(expected_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(expected_output.exists(), true);
}

#[test]
fn it_should_emit_correct_debug_asm() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let expected_output = directory.path().join("module-name.ptx");
    let reference_output = PathBuf::from("tests/codegen/outputs/debug.ptx");

    session.emit = vec![Output::PTXAssembly];
    session.output = Some(expected_output.clone());
    session.configuration = Configuration::Debug;
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.0.o"));
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.crate.metadata.o"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_math-5d32b2be875cc4d4.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_utils-315daf14970b3da5.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libcore-c8f041115f42fd27.rlib"));

    assert_eq!(expected_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(expected_output.exists(), true);
    assert_files_equal(expected_output, reference_output);
}

#[test]
fn it_should_emit_correct_release_asm() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let expected_output = directory.path().join("module-name.ptx");
    let reference_output = PathBuf::from("tests/codegen/outputs/release.ptx");

    session.emit = vec![Output::PTXAssembly];
    session.output = Some(expected_output.clone());
    session.configuration = Configuration::Release;
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.0.o"));
    session.link_bitcode(&PathBuf::from("tests/codegen/inputs/example.crate.metadata.o"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_math-5d32b2be875cc4d4.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libdummy_utils-315daf14970b3da5.rlib"));
    session.link_rlib(&PathBuf::from("tests/codegen/inputs/libcore-c8f041115f42fd27.rlib"));

    assert_eq!(expected_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(expected_output.exists(), true);
    assert_files_equal(expected_output, reference_output);
}

fn assert_files_equal(current_file_path: PathBuf, reference_file_path: PathBuf) {
    let mut current_file = BufReader::new(File::open(current_file_path).unwrap());
    let mut ref_file = BufReader::new(File::open(reference_file_path).unwrap());

    let mut current_contents = String::new();
    let mut ref_contents = String::new();

    current_file.read_to_string(&mut current_contents).unwrap();
    ref_file.read_to_string(&mut ref_contents).unwrap();

    assert_diff!(&ref_contents, &current_contents, "\n", 0);
}

