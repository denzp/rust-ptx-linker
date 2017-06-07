// llvm-link -o linked.o kernel.0.o lib-f0faab0dbaa9f7ef.0.o
// opt -strip-debug linked.o -o linked.o.opt.o
// opt -strip-debug -O3 linked.o -o linked.o.opt.o
// llc linked.o.opt.o

extern crate ptx_linker;

use std::env;
use ptx_linker::session::Session;
use ptx_linker::linker::Linker;

fn main() {
    let session = Session::from(env::args().skip(1));
    let linker = Linker::new(session);

    linker.link();
}

