# Examples

To run the examples:
```
xargo rustc --target nvptx64-nvidia-cuda --release
```

## Deep Dependencies Example: `deep-deps-example`
This example shows that the linker is actually "linking" :)

The dependencies hierarchy is:
```
example (is the CUDA kernel dylib crate)
└─ dummy_math (rlib crate)
   └─ dummy_utils (rlib crate)
```

Both `dummy_math` and `dummy_utils` exports a function and also a kernel.

A PTX output at `target/nvptx64-nvidia-cuda/release/example.ptx` should contain no `.extern` function declaration and have 3 kernels.


## Undefined Reference Example: `undefined-ref-example`
This example shows that the linker can find unresoved external references and reject linking because the output won't be a valid PTX.

When you try to run the example the linker should fail with error message:
```
[ERROR] Unable to link modules
[ERROR]   caused by: Undefined references: ["bar"]
```
