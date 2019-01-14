// TODO: temp solution for `error-chain`
#![allow(deprecated)]
#![deny(warnings)]
#![warn(clippy::all)]

#[cfg(feature = "llvm-proxy")]
extern crate rustc_llvm_proxy;

extern crate ar;
extern crate clap;
extern crate llvm_sys;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

mod llvm;
mod passes;

pub mod error;
pub mod linker;
pub mod session;

pub fn linker_entrypoint(session: session::Session) -> ! {
    use error::*;
    use linker::Linker;

    let result = {
        Linker::new(session)
            .link()
            .chain_err(|| "Unable to link modules")
    };

    let exit_code = if let Err(ref e) = result {
        error!("{}", e);

        for e in e.iter().skip(1) {
            error!("  caused by: {}", e.to_string());
        }

        if let Some(backtrace) = e.backtrace() {
            error!("{:?}", backtrace);
        }

        1
    } else {
        0
    };

    ::std::process::exit(exit_code)
}
