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
                    "tail call i32 @llvm.nvvm.read.ptx.sreg.ntid.y()",
                    "tail call i32 @llvm.nvvm.read.ptx.sreg.ctaid.y()",
                    "tail call i32 @llvm.nvvm.read.ptx.sreg.tid.y()",
                    "tail call i32 @llvm.nvvm.read.ptx.sreg.ntid.x()",
                    "tail call i32 @llvm.nvvm.read.ptx.sreg.ctaid.x()",
                    "tail call i32 @llvm.nvvm.read.ptx.sreg.tid.x()",
                ],
                &["example__Image"],
            )),
            (Profile::Debug, "examples/intrinsics") => Some((
                &[
                    "call i32 @llvm.nvvm.read.ptx.sreg.ntid.y(), !dbg",
                    "call i32 @llvm.nvvm.read.ptx.sreg.ctaid.y(), !dbg",
                    "call i32 @llvm.nvvm.read.ptx.sreg.tid.y(), !dbg",
                    "call i32 @llvm.nvvm.read.ptx.sreg.ntid.x(), !dbg",
                    "call i32 @llvm.nvvm.read.ptx.sreg.ctaid.x(), !dbg",
                    "call i32 @llvm.nvvm.read.ptx.sreg.tid.x(), !dbg",
                    "example__Image",
                ],
                &[],
            )),

            (Profile::Release, "examples/depenencies") => Some((
                &[
                    "define ptx_kernel void @dummy_math_kernel(",
                    "define ptx_kernel void @dummy_utils_kernel(",
                    "define ptx_kernel void @top_level_kernel(",
                    "define double @dummy_mul(double, double)",
                    "define double @dummy_mul_inner(double, double)",
                ],
                &[
                    "constant <{ [8 x i8] }>",
                    "define double @dummy_square(double)",
                ],
            )),
            (Profile::Debug, "examples/depenencies") => Some((
                &[
                    "define ptx_kernel void @dummy_math_kernel(",
                    "define ptx_kernel void @dummy_utils_kernel(",
                    "define ptx_kernel void @top_level_kernel(",
                    "define double @dummy_square(double)",
                    "define double @dummy_mul(double, double)",
                    "define double @dummy_mul_inner(double, double)",
                ],
                &[],
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
                    .join("deps")
                    .join("example.ll"),

                Profile::Debug => build_path
                    .join("nvptx64-nvidia-cuda")
                    .join("debug")
                    .join("deps")
                    .join("example.ll"),
            },
        )
    }
}
