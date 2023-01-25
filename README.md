RISC-V atomic emulation trap handler

A replacement trap handler to emulate the atomic extension on silicon that does not have it.

## Usage

We need to tell the Rust compiler to enable atomic code generation. We can achieve this by either setting some `rustflags`, like so

```toml
rustflags = [
# enable the atomic codegen option for RISCV
"-C", "target-feature=+a",

# tell the core library have atomics even though it's not specified in the target definition
"--cfg", 'target_has_atomic="8"',
"--cfg", 'target_has_atomic="16"',
"--cfg", 'target_has_atomic="32"',
"--cfg", 'target_has_atomic="ptr"',
]
```

or it is also possible to compile for a similiar target that has the atomic extension enabled. For example, a `riscv32imc` could use the `riscv32imac` target.

Finally, include this line in `main.rs`

```rust
use riscv_atomic_emulation_trap as _;
```

## How it works

The final binary will have (atomic) instructions that the hardware does not support;
when the hardware finds on of these instructions it will trap, this is where this crate comes in.

This crate overrides the default trap handler of the `riscv-rt` crate. By doing so it is possible to decode the instruction, check if is an instruction we can emulate,
emulate it, and finally move the pc (program counter) forward to continue on with the program. Any instructions that cannot be emulated will be reported to the
users exception handler.

Advantages of this crate

* Non-invasive. Other atomic emulation solutions require their dependancy in third party crates. However with this crate you just have to include it in your final binary.

Disadvantages of this crate

* Peformance penalty associated with context switching, emulating the instruction, then restoring the modified context. Based on limiting testing, you can expect a 2-4x slower execution compared to
**natively** supported instructions.

## Usage with riscv_rt

By default the `riscv_rt` does not store enough of the registers to perform atomic emulation when an exception occurs. You must override the trapping behavior to capture platform registers `x0-x31`. You can see an example of how this was done in [`v0.3`](https://github.com/esp-rs/riscv-atomic-emulation-trap/tree/f5eacb4b84074617e2bde7e24b780636a974fae0) of this crate