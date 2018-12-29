use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub enum Configuration {
    Release,
    Debug,
}

#[derive(Debug, PartialEq)]
pub enum Output {
    PTXAssembly,
    IntermediateRepresentation,
    Bitcode,
}

// TODO: make the fields private
#[derive(Debug, PartialEq)]
pub struct Session {
    pub output: Option<PathBuf>,
    pub include_rlibs: Vec<PathBuf>,
    pub include_bitcode_modules: Vec<PathBuf>,
    pub configuration: Configuration,
    pub emit: Vec<Output>,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            output: None,
            include_rlibs: vec![],
            include_bitcode_modules: vec![],
            configuration: Configuration::Debug,
            emit: vec![Output::PTXAssembly, Output::IntermediateRepresentation],
        }
    }
}

impl Session {
    /// Sets the output path
    pub fn set_output(&mut self, path: &Path) {
        let extension = path.extension().unwrap_or_default();

        if extension != "ptx" {
            warn!(
                "The output extension is not '.ptx'. Please consider changing from '.{}' to '.ptx'",
                extension.to_str().unwrap()
            );
        }

        self.output = Some(path.to_path_buf());
    }

    /// Sets a optimisation - debug or release
    pub fn set_configuration(&mut self, configuration: Configuration) {
        self.configuration = configuration;
    }

    /// Adds a bitcode file to the linking session
    ///
    /// **Note**, for now `*.crate.metadata.o` modules are omitted.
    pub fn link_bitcode(&mut self, path: &Path) {
        match self.is_metadata_bitcode(path) {
            true => info!("Ignoring metadata bitcode: {:?}", path),
            false => self.include_bitcode_modules.push(path.to_path_buf()),
        }
    }

    /// Adds a rlib archive to the linking session
    pub fn link_rlib(&mut self, path: &Path) {
        self.include_rlibs.push(path.to_path_buf());
    }

    fn is_metadata_bitcode(&self, path: &Path) -> bool {
        path.to_str().unwrap().ends_with(".crate.metadata.o")
    }
}
