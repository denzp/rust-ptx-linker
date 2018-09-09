use std::path::Path;
use std::process::Command;

use crate_compile_test::prelude::*;
use crate_compile_test::steps::{TestStep, TestStepFactory};

pub struct StepFactory;
pub struct Step;

impl StepFactory {
    pub fn new() -> Self {
        StepFactory {}
    }

    pub fn is_runnable() -> bool {
        if let Ok(output) = Command::new("ptxas").args(&["-V"]).output() {
            output.status.success()
        } else {
            false
        }
    }
}

impl TestStepFactory for StepFactory {
    fn initialize(&self, _config: &Config, _crate_path: &Path) -> Result<Box<TestStep>> {
        Ok(Box::new(Step {}))
    }
}

impl TestStep for Step {
    fn execute(&self, config: &Config, build_path: &Path) -> Result<()> {
        let assembly_path = match config.profile {
            Profile::Release => build_path
                .join("nvptx64-nvidia-cuda")
                .join("release")
                .join("example.ptx"),

            Profile::Debug => build_path
                .join("nvptx64-nvidia-cuda")
                .join("debug")
                .join("example.ptx"),
        };

        let output = Command::new("ptxas")
            .arg("--compile-only")
            .arg(assembly_path)
            .current_dir(build_path)
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            bail!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
