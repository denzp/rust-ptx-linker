extern crate ptx_linker;

use ptx_linker::session::*;
use std::path::Path;

#[test]
fn it_should_parse_args() {
    let args = create_args(&["-L",
                             "/rustlib/nvptx64-nvidia-cuda/lib",
                             "/kernel/target/nvptx64-nvidia-cuda/debug/deps/kernel.0.o",
                             "/kernel/target/nvptx64-nvidia-cuda/debug/deps/kernel.crate.\
                              metadata.o",
                             "-o",
                             "/kernel/target/nvptx64-nvidia-cuda/debug/deps/libkernel.ptx",
                             "-L",
                             "/kernel/target/nvptx64-nvidia-cuda/debug/deps",
                             "-L",
                             "/kernel/target/debug/deps",
                             "-L",
                             "~/rustlib/nvptx64-nvidia-cuda/lib",
                             "-Bstatic",
                             "--whole-archive",
                             "/tmp/rustc.Ew934MzC8cj0/liblib-f0faab0dbaa9f7ef.rlib",
                             "--no-whole-archive",
                             "--whole-archive",
                             "/tmp/rustc.Ew934MzC8cj0/libcore-6b4931ba2f43f84b.rlib",
                             "--no-whole-archive"]);

    let plan = Session::from(args.into_iter());

    assert_eq!(plan.output.unwrap(),
               Path::new("/kernel/target/nvptx64-nvidia-cuda/debug/deps/libkernel.ptx"));

    assert_eq!(plan.include_rlibs,
               vec![Path::new("/tmp/rustc.Ew934MzC8cj0/liblib-f0faab0dbaa9f7ef.rlib"),
                    Path::new("/tmp/rustc.Ew934MzC8cj0/libcore-6b4931ba2f43f84b.rlib")]);

    assert_eq!(plan.include_bitcode_modules,
               vec![Path::new("/kernel/target/nvptx64-nvidia-cuda/debug/deps/kernel.0.o"),
                    Path::new("/kernel/target/nvptx64-nvidia-cuda/debug/deps/kernel.crate.\
                               metadata.o")]);

    assert_eq!(plan.emit, vec![Output::PTXAssembly]);
    assert_eq!(plan.configuration, Configuration::Debug);
}

#[test]
fn it_should_parse_optimization() {
    let args = create_args(&["-o",
                             "/kernel/target/nvptx64-nvidia-cuda/debug/deps/libkernel.ptx",
                             "-O1"]);
    let plan = Session::from(args.into_iter());

    assert_eq!(plan.configuration, Configuration::Release);
}

fn create_args(args: &[&str]) -> Vec<String> {
    args.into_iter().map(|item| String::from(*item)).collect()
}

