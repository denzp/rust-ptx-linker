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
