# Rust PTX Linker
[![Build Status](https://travis-ci.org/denzp/rust-ptx-linker.svg?branch=master)](https://travis-ci.org/denzp/rust-ptx-linker)
[![Current Version](https://img.shields.io/crates/v/ptx-linker.svg)](https://crates.io/crates/ptx-linker)

## Purpose
It is definitely [possible to create](https://github.com/japaric/nvptx) CUDA (PTX) kernels written with Rust even without the linker.

You could emit PTX code with `--emit asm` flag.
Unfortunately, `--emit asm` can't link couple modules into a single PTX.
Problems comes up when you need to write more or less complex kernels, which use functions from external crates.

From [discussion](https://github.com/nagisa/math.rs/pull/3#issuecomment-304737732) another solution revealed:

1. Emit LLVM bitcode for every crate.
2. Link the bitcodes with `llvm-link`.
3. Compile output bitcode into PTX with `llc`.

The linker does the magic without the LLVM tools installed.

## Issues
According to Rust [NVPTX metabug](https://github.com/rust-lang/rust/issues/38789) it's quite realistic to solve part of bugs within this repo:

- [x] Non-inlined functions can't be used cross crate - [rust#38787](https://github.com/rust-lang/rust/issues/38787)
- [x] No "undefined reference" error is raised when it should be - [rust#38786](https://github.com/rust-lang/rust/issues/38786)

## Approach
The trick is to **build a kernels crate as "dylib"** and let the linker handle "linking".

For that, you need a special target definition json and to specify crate type in `Cargo.toml`:
``` toml
[lib]
crate_type = ["dylib"]
```

## Convinient usage
The easiest would be to rely on [ptx-builder](https://crates.io/crates/ptx-builder) to handle device crate building.
It will run `xargo` (which will invoke the linker after) and set all needed environment variables for comfortable development flow.

You can also refer to [a tutorial](https://github.com/denzp/rust-inline-cuda-tutorial/tree/master/chapter-1) about using CUDA kernels written in Rust.

## Advanced usage
Alternatively, you can use the linker solo.
First you need to install tools:
```
$ cargo install ptx-linker
$ cargo install xargo
```

Then, create a `nvptx64-nvidia-cuda` definition:
```
$ cd /path/to/kernels/crate
$ ptx-linker --print-target-json nvptx64-nvidia-cuda > nvptx64-nvidia-cuda.json
```

And finally, run a build with proper environment vars:
```
$ export RUST_TARGET_PATH="/path/to/kernels/crate"
$ xargo build --target nvptx64-nvidia-cuda --release
```

Eventually the linker will be used to produce a PTX assembly, that can be usually found at `target/nvptx64-nvidia-cuda/release/KERNELS_CRATE_NAME.ptx`.

### Target definition
The common definition for `nvptx64-nvidia-cuda` looks like:
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
    "target-pointer-width": "64",
    "target-c-int-width": "32"
}
```

Especially, the most important for the linker are the properties:
* `"linker"` - the linker executable name in `PATH`.
* `"linker-flavor"` - currently the linker supports parsing of `ld`-style arguments.
* `"linker-is-gnu"` - needed to be `true` for Rust to pass optimisation flags.
* `"dll-suffix"` - specifies a correct assembly file extension.
* `"dynamic-linking"` - allows Rust to create **dylib** for the target.
* `"obj-is-bitcode"` - store bitcode instead of object files, it's significantly easier to work with them.
