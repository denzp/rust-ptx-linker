#![deny(warnings)]
#![warn(clippy::all)]

extern crate env_logger;
extern crate error_chain;
extern crate ptx_linker;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

use env_logger::{Builder, Env};

use ptx_linker::error::*;
use ptx_linker::linker::Linker;

mod cli;
use cli::{get_cli_request, CommandLineRequest};

fn main() {
    Builder::from_env(Env::default().default_filter_or("warn")).init();

    if let Err(ref e) = run() {
        error!("{}", e);

        for e in e.iter().skip(1) {
            error!("  caused by: {}", e.to_string());
        }

        if let Some(backtrace) = e.backtrace() {
            error!("{:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    match get_cli_request() {
        CommandLineRequest::Link(session) => {
            Linker::new(session)
                .link()
                .chain_err(|| "Unable to link modules")?;
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
    };

    Ok(())
}
