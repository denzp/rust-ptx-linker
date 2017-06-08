use std::path::PathBuf;
use error::*;
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
/// Some arguments like `-o` or `-L` changes the state, we expect file path after the arguments.
/// Rust wraps rlibs with `--whole-archive` and "--no-whole-archive" arguments, so we use the fact to find them.
impl<T: IntoIterator<Item = String>> ArgsParser<T> {
    pub fn new(container: T) -> Self {
        ArgsParser {
            iterator: container.into_iter(),
            state: ParserState::Initial,
        }
    }

    pub fn create_session(mut self) -> Result<Session> {
        let mut session = Session::default();
        let mut argument = self.iterator.next();

        while argument.is_some() {
            match self.state {
                ParserState::Initial => self.state_initial(&argument.unwrap(), &mut session)?,
                ParserState::SearchPath => self.state_search_path(&argument.unwrap())?,
                ParserState::OutputPath => {
                    self.state_output_path(&argument.unwrap(), &mut session)?
                }
                ParserState::InputRlibPath => {
                    self.state_rlib_path(&argument.unwrap(), &mut session)?
                }
            }

            argument = self.iterator.next();
        }

        info!("Going to link {} bitcode modules and {} rlibs...\n",
              session.include_bitcode_modules.len(),
              session.include_rlibs.len());

        Ok(session)
    }

    fn state_initial(&mut self, argument: &str, session: &mut Session) -> Result<()> {
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
                session
                    .link_bitcode(&self.parse_path(argument)
                                       .chain_err(|| "Unexpected argument for 'Initial' state")?)
            }
        }

        Ok(())
    }

    fn state_output_path(&mut self, argument: &str, session: &mut Session) -> Result<()> {
        match argument {
            _ => {
                session.set_output(&self.parse_path(argument)
                                        .chain_err(|| "Unexpected argument for 'Output Path' state")?);

                self.state = ParserState::Initial;
            }
        }

        Ok(())
    }

    fn state_search_path(&mut self, argument: &str) -> Result<()> {
        match argument {
            _ => {
                self.parse_path(argument)
                    .chain_err(|| "Unexpected argument for 'Lib Search Path' state")?;

                self.state = ParserState::Initial;
            }
        }

        Ok(())
    }

    fn state_rlib_path(&mut self, argument: &str, session: &mut Session) -> Result<()> {
        match argument {
            "--no-whole-archive" => {
                self.state = ParserState::Initial;
            }

            _ => {
                session.link_rlib(&self.parse_path(argument)
                                       .chain_err(|| "Unexpected argument for 'Input Rlib Path' state")?);
            }
        }

        Ok(())
    }

    fn parse_path(&self, path: &str) -> Result<PathBuf> {
        if !path.starts_with("-") {
            Ok(PathBuf::from(path))
        } else {
            Err(ErrorKind::PathArgumentError(String::from(path)).into())
        }
    }
}
