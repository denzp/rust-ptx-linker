use clap::ArgMatches;
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

#[derive(Debug, PartialEq)]
pub enum CommandLineRequest {
    Link(Session),
    Print64BitTargetJson,
    Print32BitTargetJson,
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

impl<'a> From<ArgMatches<'a>> for CommandLineRequest {
    fn from(matches: ArgMatches) -> CommandLineRequest {
        match matches.subcommand_name() {
            Some("print") => {
                let target = matches
                    .subcommand_matches("print")
                    .unwrap()
                    .value_of("TARGET");

                match target {
                    Some("nvptx64-nvidia-cuda") => CommandLineRequest::Print64BitTargetJson,
                    Some("nvptx-nvidia-cuda") => CommandLineRequest::Print32BitTargetJson,

                    other => {
                        unreachable!("Unknown target: {:?}", other);
                    }
                }
            }

            _ => {
                let mut session = Session::default();

                if let Some(inputs) = matches.values_of("input") {
                    for input in inputs {
                        if input.ends_with(".o") {
                            session.link_bitcode(Path::new(input));
                        } else if input.ends_with(".rlib") {
                            session.link_rlib(Path::new(input));
                        } else {
                            warn!("Can't recognise input type: {:?}", input);
                        }
                    }
                }

                if let Some(output) = matches.value_of("output") {
                    session.set_output(Path::new(output));
                }

                match matches.value_of("optimise") {
                    Some("0") | None => session.set_configuration(Configuration::Debug),
                    Some(_) => session.set_configuration(Configuration::Release),
                };

                CommandLineRequest::Link(session)
            }
        }
    }
}
