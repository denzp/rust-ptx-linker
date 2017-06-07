use std::path::PathBuf;
use super::{Session, Configuration};

#[derive(Clone, Copy, Debug)]
enum ParserState {
    Initial,
    SearchPath,
    OutputPath,
    InputRlibPath,
}

pub struct ArgsParser<T: IntoIterator<Item = String>> {
    iterator: T::IntoIter,
    state: ParserState,
}

/// The implementation is Finite State Machine. We need to parse `ld` linker flavor.
///
/// Some arguments like "-o" or "-L" changes the state.
/// Also, from Rust sources, compiler will wrap rlibs with "--whole-archive" and "--no-whole-archive".
///
impl<T: IntoIterator<Item = String>> ArgsParser<T> {
    pub fn new(container: T) -> Self {
        ArgsParser {
            iterator: container.into_iter(),
            state: ParserState::Initial,
        }
    }

    // TODO: implement gentle error handling
    pub fn create_session(mut self) -> Session {
        let mut session = Session::default();
        let mut argument = self.iterator.next();

        while argument.is_some() {
            match self.state {
                ParserState::Initial => self.handle_initial_state(&argument.unwrap(), &mut session),
                ParserState::SearchPath => self.handle_search_path_state(&argument.unwrap()),

                ParserState::OutputPath => {
                    self.handle_output_path_state(&argument.unwrap(), &mut session)
                }
                ParserState::InputRlibPath => {
                    self.handle_input_rlib_path_state(&argument.unwrap(), &mut session)
                }
            }

            argument = self.iterator.next();
        }

        session
    }

    fn handle_initial_state(&mut self, argument: &str, session: &mut Session) {
        match argument {
            "-Bstatic" => {}
            "-Bdynamic" => {}
            "-shared" => {}

            "--whole-archive" => {
                self.state = ParserState::InputRlibPath;
            }
            "-L" => {
                self.state = ParserState::SearchPath;
            }
            "-o" => {
                self.state = ParserState::OutputPath;
            }

            "-O1" => {
                session.configuration = Configuration::Release;
            }
            _ => {
                session.link_bitcode(&self.parse_path(argument)
                                         .expect("Unknown argument for state Initial"));
            }
        }
    }

    fn handle_output_path_state(&mut self, argument: &str, session: &mut Session) {
        match argument {
            _ => {
                session.set_output(&self.parse_path(argument)
                                       .expect("Unknown argument for state OutputPath"));

                self.state = ParserState::Initial;
            }
        }
    }

    fn handle_search_path_state(&mut self, argument: &str) {
        match argument {
            _ => {
                self.parse_path(argument)
                    .expect("Path expected for state SearchPath");

                self.state = ParserState::Initial;
            }
        }
    }

    fn handle_input_rlib_path_state(&mut self, argument: &str, session: &mut Session) {
        match argument {
            "--no-whole-archive" => {
                self.state = ParserState::Initial;
            }

            _ => {
                session.link_rlib(&self.parse_path(argument)
                                      .expect("Unknown argument for state InputRlibPath"));
            }
        }
    }

    fn parse_path(&self, path: &str) -> Option<PathBuf> {
        if !path.starts_with("-") {
            Some(PathBuf::from(path))
        } else {
            None
        }
    }
}

