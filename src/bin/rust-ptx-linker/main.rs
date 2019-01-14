#![deny(warnings)]
#![warn(clippy::all)]

extern crate env_logger;
extern crate ptx_linker;

#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;

use env_logger::{Builder, Env};
use ptx_linker::linker_entrypoint;

mod cli;
use cli::current_session;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    linker_entrypoint(current_session())
}
