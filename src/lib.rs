//! RISC-V atomic emulation trap handler
//!
//! A replacement trap handler to emumlate the atomic extension on chips that do not have it in hardware, to be used in conjunction with the `riscv-rt` crate.
//! 
//! ## Usage
//!
//! As simple as including the crate in your project:
//! ```rust
//! use riscv_atomic_emulation_trap as _;
//! ```
//! 
//! ## How it works
//!
//! Instead of using the real target (non-atomic) for a given chip, it's possible to target the closest target that also has the atomic extension. For example, the `esp32c3` is
//! `riscv32imc`, therefore to use this crate you would use `riscv32imac`. The final binary will have (atomic) instructions that the hardware does not support;
//! when the hardware finds on of these instructions it will trap, this is where this crate comes in.
//!
//! This crate overrides the default trap handler of the `riscv-rt` crate. By doing so it is possible to decode the instruction, check if is an instruction we can emulate,
//! emulate it, and finally move the pc (program counter) forward to continue on with the program. Any instructions that cannot be emulated will be reported to the
//! users exception handler.
//!
//! Advantages of this crate
//! 
//! * Non-invasive. Other atomic emulation solutions require their dependancy in third party crates. However with this crate you just have to include it in your final binary.
//!
//! Disadvantages of this crate
//! 
//! * Peformance penalty associated with context switching, emulating the instruction, then restoring the modified context. Based on limiting testing, you can expect a 2-4x slower execution compared to
//! **natively** supported instructions.

#![cfg_attr(not(test), no_std)]

use riscv::{self, register::mcause};

#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub pc: usize,  // pc, x0 is useless
    pub ra: usize,  // x1
    pub sp: usize,  // x2
    pub gp: usize,  // x3
    pub tp: usize,  // x4
    pub t0: usize,  // x5
    pub t1: usize,  // x6
    pub t2: usize,  // x7
    pub fp: usize,  // x8
    pub s1: usize,  // x9
    pub a0: usize,  // x10
    pub a1: usize,  // x11
    pub a2: usize,  // x12
    pub a3: usize,  // x13
    pub a4: usize,  // x14
    pub a5: usize,  // x15
    pub a6: usize,  // x16
    pub a7: usize,  // x17
    pub s2: usize,  // x18
    pub s3: usize,  // x19
    pub s4: usize,  // x20
    pub s5: usize,  // x21
    pub s6: usize,  // x22
    pub s7: usize,  // x23
    pub s8: usize,  // x24
    pub s9: usize,  // x25
    pub s10: usize, // x26
    pub s11: usize, // x27
    pub t3: usize,  // x28
    pub t4: usize,  // x29
    pub t5: usize,  // x30
    pub t6: usize,  // x31
}

impl TrapFrame {
    unsafe fn as_mut_words(&mut self) -> &mut [usize] {
        core::slice::from_raw_parts_mut(
            self as *mut _ as *mut _,
            core::mem::size_of::<TrapFrame>() / core::mem::size_of::<usize>(),
        )
    }

    fn as_riscv_rt_trap_frame(&self) -> riscv_rt::TrapFrame {
        riscv_rt::TrapFrame {
            ra: self.ra,
            t0: self.t0,
            t1: self.t1,
            t2: self.t2,
            t3: self.t3,
            t4: self.t4,
            t5: self.t5,
            t6: self.t6,
            a0: self.a0,
            a1: self.a1,
            a2: self.a2,
            a3: self.a3,
            a4: self.a4,
            a5: self.a5,
            a6: self.a6,
            a7: self.a7,
        }
    }
}

macro_rules! amo {
    ($frame:ident, $rs1:ident, $rs2:ident, $rd:ident, $operation:expr) => {
        let tmp = $frame[$rs1];
        let a = *(tmp as *const _);
        let b = $frame[$rs2];
        $frame[$rd] = a;
        *(tmp as *mut _) = $operation(a, b);
    };
}

pub unsafe fn atomic_emulation(frame: &mut TrapFrame) -> bool {
    static mut S_LR_ADDR: usize = 0;

    // deref the addr to find the instruction we trapped on.
    let insn: usize = *(frame.pc as *const _);
    // TODO how to know if insn is executable?

    if (insn & 0b1111111) != 0b0101111 {
        return false;
    }

    let reg_mask = 0b11111;
    // destination register
    let rd = (insn >> 7) & reg_mask;
    // source 1 register
    let rs1 = (insn >> 15) & reg_mask;
    // source 2 register
    let rs2 = (insn >> 20) & reg_mask;

    let frame = frame.as_mut_words();

    match insn >> 27 {
        0b00010 => {
            /* LR */
            S_LR_ADDR = frame[rs1];
            let tmp: usize = *(S_LR_ADDR as *const _);
            frame[rd] = tmp;
        }
        0b00011 => {
            /* SC */
            let tmp: usize = frame[rs1];
            if tmp != S_LR_ADDR {
                frame[rd] = 1;
            } else {
                *(S_LR_ADDR as *mut _) = frame[rs2];
                frame[rd] = 0;
                S_LR_ADDR = 0;
            }
        }
        0b00001 => {
            /* AMOSWAP */
            amo!(frame, rs1, rs2, rd, |_, b| b);
        }
        0b00000 => {
            /* AMOADD */
            amo!(frame, rs1, rs2, rd, |a, b| a + b);
        }
        0b00100 => {
            /* AMOXOR */
            amo!(frame, rs1, rs2, rd, |a, b| a ^ b);
        }
        0b01100 => {
            /* AMOAND */
            amo!(frame, rs1, rs2, rd, |a, b| a & b);
        }
        0b01000 => {
            /* AMOOR */
            amo!(frame, rs1, rs2, rd, |a, b| a | b);
        }
        0b10000 => {
            /* AMOMIN */
            amo!(frame, rs1, rs2, rd, |a, b| (a as isize).min(b as isize));
        }
        0b10100 => {
            /* AMOMAX */
            amo!(frame, rs1, rs2, rd, |a, b| (a as isize).max(b as isize));
        }
        0b11000 => {
            /* AMOMINU */
            amo!(frame, rs1, rs2, rd, |a: usize, b| a.min(b));
        }
        0b11100 => {
            /* AMOMAXU */
            amo!(frame, rs1, rs2, rd, |a: usize, b| a.max(b));
        }
        _ => return false,
    }

    true
}

use riscv_rt::Vector;

// These are defined in the riscv-rt crate
extern "Rust" {
    #[doc(hidden)]
    pub static __INTERRUPTS: [Vector; 12];
}
extern "C" {
    fn ExceptionHandler(trap_frame: &riscv_rt::TrapFrame);
    fn DefaultHandler();
}

#[link_section = ".trap.rust"]
#[export_name = "_start_trap_atomic_rust"]
#[doc(hidden)]
pub extern "C" fn _start_trap_atomic_rust(frame: *mut TrapFrame) {
    unsafe {
        let cause = mcause::read();
        let frame = &mut *frame;
        match cause.cause() {
            mcause::Trap::Exception(e) => match e {
                mcause::Exception::IllegalInstruction => {
                    if atomic_emulation(frame) {
                        // successfull emulation, move the mepc
                        frame.pc += core::mem::size_of::<usize>();
                    } else {
                        ExceptionHandler(&frame.as_riscv_rt_trap_frame())
                    }
                }
                _ => ExceptionHandler(&frame.as_riscv_rt_trap_frame()),
            },
            mcause::Trap::Interrupt(_) => {
                let code = cause.code();
                if code < __INTERRUPTS.len() {
                    let h = &__INTERRUPTS[code];
                    if h.reserved == 0 {
                        DefaultHandler();
                    } else {
                        (h.handler)();
                    }
                } else {
                    DefaultHandler();
                }
            }
        }
    }
}

