extern crate either;

use either::Either;
use std::env::args;
use std::process::{exit, Command, Stdio};

mod iter;
use iter::IteratorExt;

fn main() {
    let mut command = Command::new("rustc");

    let status = command
        .args(transform_args(args()))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status.unwrap().code() {
        Some(code) => exit(code),
        None => exit(0),
    }
}

fn transform_args(args: impl Iterator<Item = String>) -> impl Iterator<Item = String> {
    let args = args.skip(1).collect::<Vec<_>>();

    if args.iter().find(|item| *item == "___").is_some() {
        Either::Left(args.into_iter())
    } else {
        Either::Right(args.into_iter().keep_last_pair("--crate-type".to_string()))
    }
}

#[test]
fn test_multiple_crate_types() {
    let input = &[
        "wrapper",
        "--crate-type",
        "lib",
        "--crate-name",
        "sample_crate",
        "--crate-type",
        "dylib",
        "--sysroot",
        "/tmp",
    ];

    let output = vec![
        "--crate-name".to_string(),
        "sample_crate".to_string(),
        "--sysroot".to_string(),
        "/tmp".to_string(),
        "--crate-type".to_string(),
        "dylib".to_string(),
    ];

    assert_eq!(
        transform_args(input.iter().map(|item| item.to_string())).collect::<Vec<_>>(),
        output
    );
}

#[test]
fn test_internal_discovery() {
    let input = &[
        "wrapper",
        "--crate-type",
        "lib",
        "--crate-name",
        "___",
        "--crate-type",
        "dylib",
        "--sysroot",
        "/tmp",
    ];

    let output = vec![
        "--crate-type".to_string(),
        "lib".to_string(),
        "--crate-name".to_string(),
        "___".to_string(),
        "--crate-type".to_string(),
        "dylib".to_string(),
        "--sysroot".to_string(),
        "/tmp".to_string(),
    ];

    assert_eq!(
        transform_args(input.iter().map(|item| item.to_string())).collect::<Vec<_>>(),
        output
    );
}
