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
    // TODO: warn if extension is not ".ptx"
    pub fn set_output(&mut self, path: &Path) {
        self.output = Some(path.to_path_buf());
    }

    pub fn link_bitcode(&mut self, path: &Path) {
        self.include_bitcode_modules.push(path.to_path_buf());
    }

    pub fn link_rlib(&mut self, path: &Path) {
        self.include_rlibs.push(path.to_path_buf());
    }
}

