#![feature(rustc_private)]

extern crate cty;
extern crate rustc_llvm;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

pub mod llvm;
pub mod session;
pub mod linker;
pub mod error;
