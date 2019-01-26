# Rust PTX Linker
[![Build Status](https://travis-ci.org/denzp/rust-ptx-linker.svg?branch=master)](https://travis-ci.org/denzp/rust-ptx-linker)
[![Build status](https://ci.appveyor.com/api/projects/status/fjhq7mdp1skpjfqu/branch/master?svg=true)](https://ci.appveyor.com/project/denzp/rust-ptx-linker/branch/master)
[![Current Version](https://img.shields.io/crates/v/ptx-linker.svg)](https://crates.io/crates/ptx-linker)

LLVM NVPTX bitcode linker for Rust 🔥 **without external system dependencies** 🔥!

## What's going on in v0.9?
The release is important for the linker and existing users.
Linker binaries are now split into *legacy* and *modern*.

* *Legacy* can be used with any Rust version and provides json target specification.
Basically, it has the same behaviour as prior versions.
* *Modern* uses a special CLI calling convention and requires support from Rust.
Eventually, it will become the only supported usage approach.

## Purpose
The linker solves several of issues mentioned in the [NVPTX metabug](https://github.com/rust-lang/rust/issues/38789):

- [x] Non-inlined functions can't be used cross crate - [rust#38787](https://github.com/rust-lang/rust/issues/38787)
- [x] No "undefined reference" error is raised when it should be - [rust#38786](https://github.com/rust-lang/rust/issues/38786)

## Convenient usage
The linker is rather an under-the-hood tool normally being used by Rust itself.
You just need to install it, make sure Rust is told to use the linker, and build a `cdylib` device crate.
The easiest way would be to stick with [ptx-builder](https://crates.io/crates/ptx-builder) or other device crate builder.

~~You can also refer to [a tutorial](https://github.com/denzp/rust-inline-cuda-tutorial/tree/master/chapter-1) about using CUDA kernels written in Rust.~~ *(Sorry, the tutorial is pretty outdated at the moment)*.

## Advanced usage
Alternatively, the linker can be used alone.
The modern approach uses Rust built-in target specification and a special CLI between Rust and the linker.

*Heads up! More details are coming soon!*

### Legacy approach
The approach is similar to the one was used prior `v0.9` release.
It involves `xargo` to automatically compile `libcore` for the CUDA target.

First you need to install tools:
```
$ cargo install ptx-linker
$ cargo install xargo
```

Then, create a `nvptx64-nvidia-cuda` target specification:
```
$ export RUST_TARGET_PATH="/tmp"
$ legacy-ptx-linker --print-target-json nvptx64-nvidia-cuda > $RUST_TARGET_PATH/nvptx64-nvidia-cuda.json
```

Make sure you are using a `cdylib` crate type (the step is needed to perform "linking").
Add to your `Cargo.toml`:
``` toml
[lib]
crate_type = ["cdylib"]
```

And finally, run a build with `xargo`:
```
$ cd /path/to/kernels/crate
$ xargo build --target nvptx64-nvidia-cuda --release
```

Eventually, the linker will produce a PTX assembly, that can be usually found at `target/nvptx64-nvidia-cuda/release/KERNELS_CRATE_NAME.ptx`.

## How does it work?
The linker does the magic without external system dependencies (mainly, LLVM libs) installed.
Thanks to the [rustc-llvm-proxy](https://crates.io/crates/rustc-llvm-proxy) the correct LLVM symbols are being loaded at runtime.
The approach also ensures that the linker uses same libraries versions as Rust.

### Windows users!
Unfortunately, due to [rustc-llvm-proxy#1](/denzp/rustc-llvm-proxy/issues/1) **MSVS** targets are not supported yet.

You might face similar errors:
```
Unable to find symbol 'LLVMContextCreate' in the LLVM shared lib
```

For now, the only solution on Windows is to use **GNU** host binaries.
