#![deny(warnings)]
#![warn(clippy::all)]

#[cfg(feature = "llvm-proxy")]
extern crate rustc_llvm_proxy;

mod llvm;
mod passes;

pub mod error;
pub mod linker;
pub mod session;

pub fn linker_entrypoint(session: session::Session) -> ! {
    use crate::linker::Linker;
    use log::error;

    std::process::exit(match Linker::new(session).link() {
        Ok(_) => 0,

        Err(error) => {
            error!("Unable to link modules");

            for cause in error.iter_chain() {
                error!("  caused by: {}", cause.to_string());
            }

            1
        }
    })
}
