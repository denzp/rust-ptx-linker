use std::path::PathBuf;
use tempdir::TempDir;

use ptx_linker::linker::*;
use ptx_linker::session::{Configuration, Output, Session};

#[test]
fn it_should_emit_correct_debug_asm() {
    let mut session = Session::default();
    let directory = TempDir::new("ptx-linker").unwrap();

    let actual_output = directory.path().join("module-name.ptx");

    session.emit = vec![Output::PTXAssembly];
    session.output = Some(actual_output.clone());
    session.configuration = Configuration::Debug;
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/debug/inputs/example.4jffkl93bkv60fxe.rcgu.o",
    ));
    session.link_bitcode(&PathBuf::from(
        "tests/codegen/debug/inputs/example.3912kojcc92n58xs.rcgu.o",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/debug/inputs/libnvptx_builtins-34b89b2693088fee.rlib",
    ));
    session.link_rlib(&PathBuf::from(
        "tests/codegen/debug/inputs/libcore-f286884009a67e94.rlib",
    ));

    assert_eq!(actual_output.exists(), false);
    Linker::new(session).link().unwrap();

    assert_eq!(actual_output.exists(), true);
    assert_file_not_contains!(actual_output, ["example..Image", "example..MutImage", ".."]);
    assert_file_contains!(actual_output, ["example__Image", "example__MutImage"]);
}
