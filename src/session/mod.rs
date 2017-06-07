use std::path::{PathBuf, Path};

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

// TODO: documentation
// TODO: rewrite me
impl<T: Iterator<Item = String>> From<T> for Session {
    fn from(args: T) -> Self {
        let mut plan = Session::default();

        let mut iter = args;
        let mut next = iter.next();

        while next.is_some() {
            let argument = next.unwrap();

            match argument.as_ref() {
                "-Bstatic" |
                "-Bdynamic" |
                "-shared" |
                "--whole-archive" |
                "--no-whole-archive" => {
                    // For our simple case, we don't need to handle these args
                }

                "-O1" => {
                    plan.configuration = Configuration::Release;
                }
                "-L" => {
                    iter.next().expect("Expected path after '-L' argument");
                }
                "-o" => {
                    plan.set_output(
                        Path::new(&iter.next().expect("Expected path after '-o' argument"))
                    );
                }
                _ => {
                    plan.link_file(Path::new(&argument));
                }
            }

            next = iter.next();
        }

        plan
    }
}

impl Session {
    // TODO: warn if extension is not ".ptx"
    pub fn set_output(&mut self, path: &Path) {
        self.output = Some(path.to_path_buf());
    }

    // TODO: find better logic
    pub fn link_file(&mut self, path: &Path) {
        match path.extension() {
            Some(extension) => {
                match extension.to_str().unwrap() {
                    "o" => self.link_bitcode(path),
                    "rlib" => self.link_rlib(path),

                    _ => panic!("Unknown file type for linking '{}'", path.to_str().unwrap()),
                }
            }

            None => panic!("No handler found for argument '{}'", path.to_str().unwrap()),
        }
    }

    pub fn link_bitcode(&mut self, path: &Path) {
        self.include_bitcode_modules.push(path.to_path_buf());
    }

    pub fn link_rlib(&mut self, path: &Path) {
        self.include_rlibs.push(path.to_path_buf());
    }
}

