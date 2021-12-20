RISC-V atomic emulation trap handler

A replacement trap handler to emumlate the atomic extension on chips that do not have it in hardware, to be used in conjunction with the `riscv-rt` crate.

## Usage

As simple as including the crate in your project:
```rust
use riscv_atomic_emulation_trap as _;
```

## How it works

Instead of using the real target (non-atomic) for a given chip, it's possible to target the closest target that also has the atomic extension. For example, the `esp32c3` is
`riscv32imc`, therefore to use this crate you would use `riscv32imac`. The final binary will have (atomic) instructions that the hardware does not support;
when the hardware finds on of these instructions it will trap, this is where this crate comes in.

This crate overrides the default trap handler of the `riscv-rt` crate. By doing so it is possible to decode the instruction, check if is an instruction we can emulate,
emulate it, and finally move the pc (program counter) forward to continue on with the program. Any instructions that cannot be emulated will be reported to the
users exception handler.

Advantages of this crate

* Non-invasive. Other atomic emulation solutions require their dependancy in third party crates. However with this crate you just have to include it in your final binary.

Disadvantages of this crate

* Peformance penalty associated with context switching, emulating the instruction, then restoring the modified context. Based on limiting testing, you can expect a 2-4x slower execution compared to
**natively** supported instructions.