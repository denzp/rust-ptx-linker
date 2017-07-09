# Rust PTX Linker
[![Build Status](https://travis-ci.org/denzp/rust-ptx-linker.svg?branch=master)](https://travis-ci.org/denzp/rust-ptx-linker)

## Purpose
For some time, even without the linker, it is [possible to create](https://github.com/japaric/nvptx) CUDA (PTX) kernels written with Rust. 

The one could emit PTX code with `--emit asm` flag. But some problems come up when we need to write more or less complex kernels, which uses functions from external crates.

Unfortunately, `--emit asm` can't link couple modules into a single PTX. From [dicsussion](https://github.com/nagisa/math.rs/pull/3#issuecomment-304737732) another solution revealed:

1. Emit LLVM bitcode for every crate.
1. Link the bitcodes with `llvm-link`.
1. Compile output bitcode into PTX with `llc`.

## Issues
According to Rust [NVPTX metabug](https://github.com/rust-lang/rust/issues/38789) it's quite realistic to solve part of bugs within this repo:

- [x] Non-inlined functions can't be used cross crate - [rust#38787](https://github.com/rust-lang/rust/issues/38787)
- [x] No "undefined reference" error is raised when it should be - [rust#38786](https://github.com/rust-lang/rust/issues/38786)


## Approach

**The trick it to compile kernels crate as *dylib*.**

So you usually have to add to your `Cargo.toml`:
``` toml
[lib]
crate_type = ["dylib"]
```

And also, some modifications has to be made for target definition:
``` json
{
    "arch": "nvptx64",
    "cpu": "sm_20",
    "data-layout": "e-i64:64-v16:16-v32:32-n16:32:64",
    "linker": "ptx-linker",
    "linker-flavor": "ld",
    "linker-is-gnu": true,
    "dll-prefix": "",
    "dll-suffix": ".ptx",
    "dynamic-linking": true,
    "llvm-target": "nvptx64-nvidia-cuda",
    "max-atomic-width": 0,
    "os": "cuda",
    "obj-is-bitcode": true,
    "panic-strategy": "abort",
    "target-endian": "little",
    "target-pointer-width": "64"
}
```

Especially, the most important for the linker:
* `"linker": "ptx-linker"` - the linker executable in `PATH`.
* `"linker-flavor": "ld"` - currently we support only `ld` flavor parsing.
* `"linker-is-gnu": true` - it needs for Rust to pass optimisation flag.
* `"dll-suffix": ".ptx"` - correct file extension for PTX assembly output.
* `"dynamic-linking": true` - allows Rust to create **dylib**.
* `"obj-is-bitcode": true` - store bitcode instead of object files.

After that you can:
```
$ echo "Installing PTX linker"
$ cargo install ptx-linker

$ cd /path/to/kernels/crate
$ echo "Building PTX assembly output"
$ xargo rustc --target nvptx64-nvidia-cuda --release
```

---

We are not going to run any LLVM tools, because they are unlikely in `PATH` or their version is not the same as Rust's LLVM. What we are going to do, is to use LLVM api here.

It's possible to use the api thanks to `#![feature(rustc_private)]`. To be honest, I didn't know before that we can use the feature not only for a compiler plugin, but also for a usual crates :)

But unfortunately it means we are stick to **nightly** Rust. Probably later we might find another approach that works also for **stable**.