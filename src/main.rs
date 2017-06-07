// 1. llvm-link -o linked.o kernel.0.o lib-f0faab0dbaa9f7ef.0.o
// 2. opt -strip-debug linked.o -o linked.o.opt.o
// 2. opt -strip-debug -O3 linked.o -o linked.o.opt.o
// 3. llc linked.o.opt.o

extern crate ptx_linker;

use std::env;
use ptx_linker::session::ArgsParser;
use ptx_linker::linker::Linker;

fn main() {
    let session = ArgsParser::new(env::args().skip(1)).create_session();
    let linker = Linker::new(session);

    linker.link();
}

