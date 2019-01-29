use std::path::{Path, PathBuf};

use log::*;

#[derive(Debug, PartialEq)]
pub enum OptLevel {
    None,
    LTO,
}

impl Default for OptLevel {
    fn default() -> Self {
        OptLevel::None
    }
}

#[derive(Debug, PartialEq)]
pub enum Output {
    PTXAssembly,
    IntermediateRepresentation,
    Bitcode,
}

// TODO: make the fields private
#[derive(Debug, PartialEq, Default)]
pub struct Session {
    pub output: Option<PathBuf>,
    pub include_rlibs: Vec<PathBuf>,
    pub include_bitcode_modules: Vec<PathBuf>,

    pub opt_level: OptLevel,
    pub debug_info: bool,

    pub emit: Vec<Output>,
    pub ptx_archs: Vec<String>,
    pub ptx_fallback_arch: String,
}

impl Session {
    /// Sets the output path.
    pub fn set_output(&mut self, path: &Path) {
        let extension = path.extension().unwrap_or_default();

        if extension != "ptx" {
            warn!(
                "The output extension is not '.ptx'. Consider changing from '.{}' to '.ptx'",
                extension.to_str().unwrap()
            );
        }

        self.output = Some(path.to_path_buf());
    }

    /// Sets an optimisation level.
    pub fn set_opt_level(&mut self, level: OptLevel) {
        self.opt_level = level;
    }

    /// Emit debug information or not.
    pub fn set_debug_info(&mut self, debug: bool) {
        self.debug_info = debug;
    }

    /// Adds a bitcode file to the linking session.
    ///
    /// **Note**, for now `*.crate.metadata.o` modules are omitted.
    pub fn link_bitcode(&mut self, path: &Path) {
        if self.is_metadata_bitcode(path) {
            info!("Ignoring metadata bitcode: {:?}", path)
        } else {
            self.include_bitcode_modules.push(path.to_path_buf());
        }
    }

    /// Adds a rlib archive to the linking session.
    pub fn link_rlib(&mut self, path: &Path) {
        self.include_rlibs.push(path.to_path_buf());
    }

    /// Emit artifacts of the type.
    pub fn add_output_type(&mut self, output: Output) {
        self.emit.push(output);
    }

    /// Specify output architecture (e.g. `sm_60`).
    pub fn add_output_arch(&mut self, arch: &str) {
        self.ptx_archs.push(arch.into());
    }

    /// Specify the fallback architecture if no other explicitly set.
    pub fn set_fallback_arch(&mut self, arch: &str) {
        self.ptx_fallback_arch = arch.into();
    }

    fn is_metadata_bitcode(&self, path: &Path) -> bool {
        path.to_str().unwrap().ends_with(".crate.metadata.o")
    }
}
