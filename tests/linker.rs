use std::env::{current_dir, current_exe};
use std::path::Path;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate crate_compile_test;

use crate_compile_test::prelude::*;
mod steps;

crate_compile_test_suite! {
    "Debug PTX Assembly" => {
        let mut config = create_config(Mode::BuildSuccess, Profile::Debug);

        config
            .additional_steps
            .push(Box::new(steps::assembly::StepFactory::new()));

        run_compile_tests!(config);
    },

    "Release PTX Assembly" => {
        let mut config = create_config(Mode::BuildSuccess, Profile::Release);

        config
            .additional_steps
            .push(Box::new(steps::assembly::StepFactory::new()));

        run_compile_tests!(config);
    },

    "Debug IR" => {
        let mut config = create_config(Mode::BuildSuccess, Profile::Debug);

        config
            .additional_steps
            .push(Box::new(steps::ir::StepFactory::new()));

        run_compile_tests!(config);
    },

    "Release IR" => {
        let mut config = create_config(Mode::BuildSuccess, Profile::Release);

        config
            .additional_steps
            .push(Box::new(steps::ir::StepFactory::new()));

        run_compile_tests!(config);
    },

    "Debug linking fail" => {
        run_compile_tests!(create_config(Mode::BuildFail, Profile::Debug));
    },

    "Release linking fail" => {
        run_compile_tests!(create_config(Mode::BuildFail, Profile::Release));
    }
}

fn create_config(mode: Mode, profile: Profile) -> Config {
    let mut config = Config::new(mode, "examples");

    config.cargo_command = "xargo".into();
    config.profile = profile;
    config.target = Some("nvptx64-nvidia-cuda".into());

    match config.mode {
        Mode::BuildFail => {
            config.crates_filter = Box::new(|path| path == Path::new("examples/undefined-ref"));
        }

        Mode::BuildSuccess => {
            config.crates_filter = Box::new(|path| path != Path::new("examples/undefined-ref"));
        }

        _ => unimplemented!(),
    };

    config.add_cargo_env(
        "CARGO_TARGET_NVPTX64_NVIDIA_CUDA_LINKER",
        &current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("ptx-linker")
            .to_string_lossy(),
    );

    config.add_cargo_env(
        "RUST_TARGET_PATH",
        &current_dir().unwrap().join("targets").to_string_lossy(),
    );

    config
}
