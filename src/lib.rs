extern crate ar;
extern crate cty;
extern crate llvm_sys;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

mod llvm;
mod llvm_legacy;
mod passes;

pub mod session;
pub mod linker;
pub mod error;
