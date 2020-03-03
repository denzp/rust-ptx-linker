# Rust PTX Linker
[![Build Status](https://travis-ci.org/denzp/rust-ptx-linker.svg?branch=master)](https://travis-ci.org/denzp/rust-ptx-linker)
[![Build status](https://ci.appveyor.com/api/projects/status/fjhq7mdp1skpjfqu/branch/master?svg=true)](https://ci.appveyor.com/project/denzp/rust-ptx-linker/branch/master)
[![Current Version](https://img.shields.io/crates/v/ptx-linker.svg)](https://crates.io/crates/ptx-linker)

LLVM NVPTX bitcode linker for Rust 🔥 **without external system dependencies** 🔥!

## What's going on in v0.9?
The release is important for the linker and existing users.
The former approach was using an external `nvptx64-nvidia-cuda` json target specification and `xargo` to automatically compile `libcore`.

As of *2019-02-06* Rust [received built-in support](https://github.com/rust-lang/rust/pull/57937) for building the CUDA kernels, and which evolved from the experience gained with `ptx-linker` prior `v0.9`.

Currently, it's possible to jump into a CUDA development with Nightly Rust:

``` bash
# Install the minimal required version of the linker.
$ cargo install ptx-linker -f --version ">= 0.9"

# Install `libcore` for the CUDA target.
$ rustup target add nvptx64-nvidia-cuda
```

More details about further usage can be found below ([**Advanced usage**](#advanced-usage) section).

## Purpose
The linker solves several of issues mentioned in the [NVPTX metabug](https://github.com/rust-lang/rust/issues/38789):

- [x] Non-inlined functions can't be used cross crate - [rust#38787](https://github.com/rust-lang/rust/issues/38787)
- [x] No "undefined reference" error is raised when it should be - [rust#38786](https://github.com/rust-lang/rust/issues/38786)

## Convenient usage
At the moment [ptx-builder](https://crates.io/crates/ptx-builder) is recommended approach to build Rust crates that contains CUDA code.

## Advanced usage
Alternatively, the linker can be used alone.

Make sure you are using a `cdylib` crate type (the step is needed to perform the actual "linking").
Add to your `Cargo.toml`:
``` toml
[lib]
crate_type = ["cdylib"]
```

And finally, build the PTX assembly file:
``` bash
$ cd /path/to/kernels/crate
$ cargo build --target nvptx64-nvidia-cuda --release
```

Rust will involve `ptx-linker` under-the-hood and the latter will write the assembly at:
```
target/nvptx64-nvidia-cuda/release/KERNELS_CRATE_NAME.ptx
```

## How does it work?
The linker does the magic without external system dependencies (mainly, LLVM libs) installed.
Thanks to the [rustc-llvm-proxy](https://crates.io/crates/rustc-llvm-proxy) the correct LLVM symbols are being loaded at runtime.
The approach also ensures that the linker uses same libraries versions as Rust.

### Windows users!
Unfortunately, due to LLVM dylib limitations, Windows targets are not supported.
The issue can be worked around if the linker is built with a static LLVM, but this requires a tighter integration with the Rust build process.
Currently, there is no work is done in this direction, but the situation might change.

