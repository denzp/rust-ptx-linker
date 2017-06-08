use std::path::{PathBuf, Path};

mod args_parser;
pub use self::args_parser::ArgsParser;

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
            emit: vec![Output::PTXAssembly],
        }
    }
}

impl Session {
    /// Sets the output path
    pub fn set_output(&mut self, path: &Path) {
        let extension = path.extension().unwrap();

        if extension != "ptx" {
            warn!("The output extension is not '.ptx'. Please consider changing from '.{}' to '.ptx'",
                  extension.to_str().unwrap());
        }

        self.output = Some(path.to_path_buf());
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
    ///
    /// **Note**, because of LLVM assertions `libcore` is omitted.
    pub fn link_rlib(&mut self, path: &Path) {
        match self.is_libcore_rlib(path) {
            true => info!("Ignoring libcore rlib: {:?}", path),
            false => self.include_rlibs.push(path.to_path_buf()),
        }
    }

    fn is_metadata_bitcode(&self, path: &Path) -> bool {
        path.to_str().unwrap().ends_with(".crate.metadata.o")
    }

    fn is_libcore_rlib(&self, path: &Path) -> bool {
        path.to_str().unwrap().contains("libcore")
    }
}
