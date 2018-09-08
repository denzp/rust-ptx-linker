use std::path::{Path, PathBuf};

use crate_compile_test::prelude::*;
use crate_compile_test::steps::{TestStep, TestStepFactory};

use super::LinkOutputCheckStep;

pub struct StepFactory;
pub struct Step {
    crate_path: PathBuf,
}

impl StepFactory {
    pub fn new() -> Self {
        StepFactory {}
    }
}

impl TestStepFactory for StepFactory {
    fn initialize(&self, _config: &Config, crate_path: &Path) -> Result<Box<TestStep>> {
        Ok(Box::new(Step {
            crate_path: crate_path.into(),
        }))
    }
}

impl LinkOutputCheckStep for Step {
    fn get_crate_name(&self) -> String {
        self.crate_path.to_string_lossy().into()
    }

    fn get_content(&self, profile: &Profile, path: &str) -> Option<(&[&str], &[&str])> {
        match (profile, path.replace("\\", "/").as_str()) {
            (Profile::Release, "examples/intrinsics") => Some((
                &[
                    "%ntid.y;",
                    "%ctaid.y;",
                    "%tid.y;",
                    "%ntid.x;",
                    "%ctaid.x;",
                    "%tid.x;",
                ],
                &["example__Image"],
            )),
            (Profile::Debug, "examples/intrinsics") => Some((
                &[
                    "%ntid.y;",
                    "%ctaid.y;",
                    "%tid.y;",
                    "%ntid.x;",
                    "%ctaid.x;",
                    "%tid.x;",
                    "example__image__Image",
                ],
                &["example..image..Image"],
            )),

            (Profile::Release, "examples/depenencies") => Some((
                &[
                    ".visible .entry dummy_math_kernel(",
                    ".visible .entry dummy_utils_kernel(",
                    ".visible .entry top_level_kernel(",
                    ".func  (.param .b64 func_retval0) dummy_square(",
                    ".func  (.param .b64 func_retval0) dummy_mul(",
                    ".func  (.param .b64 func_retval0) dummy_mul_inner(",
                    ".func  (.param .b64 func_retval0) dummy_mul_2(",
                ],
                &[".global .align 4 .b8"],
            )),
            (Profile::Debug, "examples/depenencies") => Some((
                &[
                    "_0[8] = {0, 0, 160, 64, 0, 0, 224, 64}",
                    ".visible .entry dummy_math_kernel(",
                    ".visible .entry dummy_utils_kernel(",
                    ".visible .entry top_level_kernel(",
                    ".func  (.param .b64 func_retval0) dummy_square(",
                    ".func  (.param .b64 func_retval0) dummy_mul(",
                    ".func  (.param .b64 func_retval0) dummy_mul_inner(",
                    ".func  (.param .b64 func_retval0) dummy_mul_2(",
                ],
                &[".0"],
            )),

            _ => None,
        }
    }
}

impl TestStep for Step {
    fn execute(&self, config: &Config, build_path: &Path) -> Result<()> {
        self.check_output(
            config,
            &match config.profile {
                Profile::Release => build_path
                    .join("nvptx64-nvidia-cuda")
                    .join("release")
                    .join("example.ptx"),

                Profile::Debug => build_path
                    .join("nvptx64-nvidia-cuda")
                    .join("debug")
                    .join("example.ptx"),
            },
        )
    }
}
