#![deny(warnings)]
#![warn(clippy::all)]

extern crate env_logger;
extern crate ptx_linker;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

use env_logger::{Builder, Env};
use ptx_linker::linker_entrypoint;

mod cli;
use crate::cli::{get_cli_request, CommandLineRequest};

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    match get_cli_request() {
        CommandLineRequest::Link(session) => {
            linker_entrypoint(session);
        }

        CommandLineRequest::Print64BitTargetJson => {
            println!(
                "{}",
                include_str!("../../../targets/nvptx64-nvidia-cuda.json")
            );
        }

        CommandLineRequest::Print32BitTargetJson => {
            println!(
                "{}",
                include_str!("../../../targets/nvptx-nvidia-cuda.json")
            );
        }
    }
}
