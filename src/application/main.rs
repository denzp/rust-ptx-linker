extern crate colored;
extern crate error_chain;
extern crate fern;
extern crate ptx_linker;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;

mod logging;
use logging::{setup_logging, AlignedOutputString};

use clap::App;
use ptx_linker::error::*;
use ptx_linker::linker::Linker;
use ptx_linker::session::CommandLineRequest;

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
    let yaml = load_yaml!("../../cli.yml");
    let matches = App::from_yaml(yaml)
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();

    match CommandLineRequest::from(matches) {
        CommandLineRequest::Link(session) => {
            Linker::new(session)
                .link()
                .chain_err(|| "Unable to link modules")?;
        }

        CommandLineRequest::PrintTargetJson => {
            println!("{}", include_str!("../../targets/nvptx64-nvidia-cuda.json"));
        }
    };

    Ok(())
}
