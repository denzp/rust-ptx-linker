#[macro_use]
extern crate log;
extern crate error_chain;
extern crate fern;
extern crate colored;

extern crate ptx_linker;

mod logging;
use logging::{AlignedOutputString, setup_logging};

use std::env;
use ptx_linker::session::ArgsParser;
use ptx_linker::linker::Linker;
use ptx_linker::error::*;

fn main() {
    setup_logging();

    if let Err(ref e) = run() {
        error!("{}", e);

        for e in e.iter().skip(1) {
            error!("  caused by: {}", e.to_string().prefix_with_spaces(13));
        }

        if let Some(backtrace) = e.backtrace() {
            error!("{:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let session = ArgsParser::new(env::args().skip(1))
        .create_session()
        .chain_err(|| "Unable to create a session")?;

    Linker::new(session)
        .link()
        .chain_err(|| "Unable to link modules")
}
