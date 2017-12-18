use std::path::PathBuf;
use tempdir::TempDir;

use ptx_linker::linker::*;
use ptx_linker::session::{Configuration, Output, Session};

#[test]
fn it_should_reject_to_emit_when_undefined_ref_exists() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let expected_output = directory.path().join("module-name.ptx");

    session.emit = vec![Output::PTXAssembly];
    session.output = Some(expected_output.clone());
    session.configuration = Configuration::Debug;
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/undefined_ref/inputs/example.0.o",
    ));

    assert_eq!(expected_output.exists(), false);
    let result = Linker::new(session).link();

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Undefined references: [\"bar\"]"
    );

    assert_eq!(expected_output.exists(), false);
}
