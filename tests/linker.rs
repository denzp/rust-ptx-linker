extern crate crate_compile_test;

use std::env::{current_dir, current_exe, var};
use std::path::Path;

use crate_compile_test::prelude::*;
use crate_compile_test::steps::{TestStep, TestStepFactory};

#[macro_use]
mod helpers;

// TODO: custom PTX verification step

#[test]
fn run_debug_success_compilation_tests() {
    let mut config = Config::new(Mode::BuildSuccess, "examples");

    config.cargo_command = "xargo".into();
    config.target = Some("nvptx64-nvidia-cuda".into());
    config.profile = Profile::Debug;
    config.crates_filter = Box::new(|path| path != Path::new("examples/undefined-ref"));

    config.add_cargo_env(
        "PATH",
        &format!("{}:{}", get_build_dir!(), var("PATH").unwrap()),
    );
    config.add_cargo_env(
        "RUST_TARGET_PATH",
        &current_dir().unwrap().join("targets").to_string_lossy(),
    );

    run_tests(config);
}

#[test]
fn run_release_success_compilation_tests() {
    let mut config = Config::new(Mode::BuildSuccess, "examples");

    config.cargo_command = "xargo".into();
    config.target = Some("nvptx64-nvidia-cuda".into());
    config.profile = Profile::Release;
    config.crates_filter = Box::new(|path| path != Path::new("examples/undefined-ref"));

    config.add_cargo_env(
        "PATH",
        &format!("{}:{}", get_build_dir!(), var("PATH").unwrap()),
    );
    config.add_cargo_env(
        "RUST_TARGET_PATH",
        &current_dir().unwrap().join("targets").to_string_lossy(),
    );

    run_tests(config);
}

#[test]
fn run_debug_fail_compilation_tests() {
    let mut config = Config::new(Mode::BuildFail, "examples");

    config.cargo_command = "xargo".into();
    config.target = Some("nvptx64-nvidia-cuda".into());
    config.profile = Profile::Debug;
    config.crates_filter = Box::new(|path| path == Path::new("examples/undefined-ref"));

    config.add_cargo_env(
        "PATH",
        &format!("{}:{}", get_build_dir!(), var("PATH").unwrap()),
    );
    config.add_cargo_env(
        "RUST_TARGET_PATH",
        &current_dir().unwrap().join("targets").to_string_lossy(),
    );

    run_tests(config);
}

#[test]
fn run_release_fail_compilation_tests() {
    let mut config = Config::new(Mode::BuildFail, "examples");

    config.cargo_command = "xargo".into();
    config.target = Some("nvptx64-nvidia-cuda".into());
    config.profile = Profile::Release;
    config.crates_filter = Box::new(|path| path == Path::new("examples/undefined-ref"));

    config.add_cargo_env(
        "PATH",
        &format!("{}:{}", get_build_dir!(), var("PATH").unwrap()),
    );
    config.add_cargo_env(
        "RUST_TARGET_PATH",
        &current_dir().unwrap().join("targets").to_string_lossy(),
    );

    run_tests(config);
}
