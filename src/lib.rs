#![deny(warnings)]

extern crate ar;
extern crate clap;
extern crate llvm_sys;
extern crate rustc_llvm_proxy;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

mod llvm;
mod passes;

pub mod error;
pub mod linker;
pub mod session;
