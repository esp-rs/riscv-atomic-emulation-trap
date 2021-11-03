#![no_std]

use riscv;

use core::fmt::Write;

#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug)]
pub struct TrapFrame {
    pub zero: usize, // x0
    pub ra: usize,   // x1
    pub sp: usize,   // x2
    pub gp: usize,   // x3
    pub tp: usize,   // x4
    pub t0: usize,   // x5
    pub t1: usize,   // x6
    pub t2: usize,   // x7
    pub fp: usize,   // x8
    pub s1: usize,   // x9
    pub a0: usize,   // x10
    pub a1: usize,   // x11
    pub a2: usize,   // x12
    pub a3: usize,   // x13
    pub a4: usize,   // x14
    pub a5: usize,   // x15
    pub a6: usize,   // x16
    pub a7: usize,   // x17
    pub s2: usize,   // x18
    pub s3: usize,   // x19
    pub s4: usize,   // x20
    pub s5: usize,   // x21
    pub s6: usize,   // x22
    pub s7: usize,   // x23
    pub s8: usize,   // x24
    pub s9: usize,   // x25
    pub s10: usize,  // x26
    pub s11: usize,  // x27
    pub t3: usize,   // x28
    pub t4: usize,   // x29
    pub t5: usize,   // x30
    pub t6: usize,   // x31
}

impl TrapFrame {
    unsafe fn as_mut_words(&mut self) -> &mut [usize] {
        core::slice::from_raw_parts_mut(
            self as *mut _ as *mut _,
            core::mem::size_of::<TrapFrame>(),
        )
    }
}

pub unsafe fn atomic_emulation(frame: &mut TrapFrame) -> bool {
    static mut S_LR_ADDR: usize = 0;

    let mepc = riscv::register::mepc::read();
    // deref the addr to find the instruction we trapped on.
    let insn: usize = *(mepc as *const _);
    // TODO how to know if insn is executable?

    if (insn & 0b1111111) != 0b0101111 {
        return false;
    }

    let reg_mask = 0b11111;
    let rd = (insn >> 7) & reg_mask;
    let rs1 = (insn >> 15) & reg_mask;
    let rs2 = (insn >> 20) & reg_mask;

    let frame = frame.as_mut_words();

    writeln!(Uart, "RD({}) = {}", rd, frame[rd]).ok();
    writeln!(Uart, "RS1({}) = {}", rs1, frame[rs1]).ok();
    writeln!(Uart, "RS2({}) = {}", rs2, frame[rs2]).ok();

    match insn >> 27 {
        0b00010 => {
            /* LR */
            writeln!(Uart, "Emulating LR").ok();
            S_LR_ADDR = frame[rs1];
            let tmp: usize = *(S_LR_ADDR as *const _);
            writeln!(Uart, "tmp = {}", tmp).ok();
            frame[rd] = tmp;
        }
        0b00011 => {
            /* SC */
            writeln!(Uart, "Emulating SC").ok();
            let tmp: usize = frame[rs1];
            if tmp != S_LR_ADDR {
                frame[rd] = 1;
            } else {
                *(S_LR_ADDR as *mut _) = frame[rs2];
                frame[rd] = 0;
                S_LR_ADDR = 0;
            }
        }
        _ => return false,
    }

    true
}

// TODO override _start_trap in riscv-rt

// TODO this is giga UB, instead make trapframe mutable upstream

// TODO remove this
pub struct Uart;

extern "C" {
    pub fn uart_tx_one_char(byte: u8) -> i32;
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(for &b in s.as_bytes() {
            unsafe { uart_tx_one_char(b) };
        })
    }
}
