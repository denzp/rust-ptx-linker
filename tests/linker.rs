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
    let mut plan = Session::default();

    let directory = TempDir::new("ptx-linker-debug").unwrap();
    plan.output = Some(directory.path().join("test-module-name.o"));

    plan.configuration = Configuration::Debug;
    plan.emit = vec![Output::IntermediateRepresentation];
    plan.include_bitcode_modules = vec![PathBuf::from("tests/codegen/input.kernel.bc")];
    plan.include_rlibs = vec![PathBuf::from("tests/codegen/input.core.rlib"),
                              PathBuf::from("tests/codegen/input.lib.rlib")];

    let linker = Linker::new(plan);

    assert_eq!(directory.path().join("test-module-name.ll").exists(), false);
    linker.link();

    assert_eq!(directory.path().join("test-module-name.ll").exists(), true);
    assert_files_equal(directory.path().join("test-module-name.ll"),
                       PathBuf::from("tests/codegen/output.debug.ll"));
}

#[test]
fn it_should_emit_correct_release_ir() {
    let mut plan = Session::default();

    let directory = TempDir::new("ptx-linker-release").unwrap();
    plan.output = Some(directory.path().join("test-module-name.o"));

    plan.configuration = Configuration::Release;
    plan.emit = vec![Output::IntermediateRepresentation];
    plan.include_bitcode_modules = vec![PathBuf::from("tests/codegen/input.kernel.bc")];
    plan.include_rlibs = vec![PathBuf::from("tests/codegen/input.core.rlib"),
                              PathBuf::from("tests/codegen/input.lib.rlib")];

    let linker = Linker::new(plan);

    assert_eq!(directory.path().join("test-module-name.ll").exists(), false);
    linker.link();

    assert_eq!(directory.path().join("test-module-name.ll").exists(), true);
    assert_files_equal(directory.path().join("test-module-name.ll"),
                       PathBuf::from("tests/codegen/output.release.ll"));
}

#[test]
fn it_should_emit_bc() {
    let mut plan = Session::default();

    let directory = TempDir::new("ptx-linker-debug").unwrap();
    plan.output = Some(directory.path().join("test-module-name.o"));

    plan.emit = vec![Output::Bitcode];
    plan.include_bitcode_modules = vec![PathBuf::from("tests/codegen/input.kernel.bc")];
    plan.include_rlibs = vec![PathBuf::from("tests/codegen/input.core.rlib"),
                              PathBuf::from("tests/codegen/input.lib.rlib")];

    let linker = Linker::new(plan);

    assert_eq!(directory.path().join("test-module-name.bc").exists(), false);
    linker.link();

    assert_eq!(directory.path().join("test-module-name.bc").exists(), true);
}

#[test]
fn it_should_emit_correct_debug_asm() {
    let mut plan = Session::default();

    let directory = TempDir::new("ptx-linker-debug").unwrap();
    plan.output = Some(directory.path().join("module-name.o"));

    plan.configuration = Configuration::Debug;
    plan.include_bitcode_modules = vec![PathBuf::from("tests/codegen/input.kernel.bc")];
    plan.include_rlibs = vec![PathBuf::from("tests/codegen/input.core.rlib"),
                              PathBuf::from("tests/codegen/input.lib.rlib")];

    let linker = Linker::new(plan);

    assert_eq!(directory.path().join("module-name.ptx").exists(), false);
    linker.link();

    assert_eq!(directory.path().join("module-name.ptx").exists(), true);
    assert_files_equal(directory.path().join("module-name.ptx"),
                       PathBuf::from("tests/codegen/output.debug.ptx"));
}

#[test]
fn it_should_emit_correct_release_asm() {
    let mut plan = Session::default();

    let directory = TempDir::new("ptx-linker-release").unwrap();
    plan.output = Some(directory.path().join("module-name.o"));

    plan.configuration = Configuration::Release;
    plan.include_bitcode_modules = vec![PathBuf::from("tests/codegen/input.kernel.bc")];
    plan.include_rlibs = vec![PathBuf::from("tests/codegen/input.core.rlib"),
                              PathBuf::from("tests/codegen/input.lib.rlib")];

    let linker = Linker::new(plan);

    assert_eq!(directory.path().join("module-name.ptx").exists(), false);
    linker.link();

    assert_eq!(directory.path().join("module-name.ptx").exists(), true);
    assert_files_equal(directory.path().join("module-name.ptx"),
                       PathBuf::from("tests/codegen/output.release.ptx"));
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

