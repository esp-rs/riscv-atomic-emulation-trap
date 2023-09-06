#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

pub const PLATFORM_REGISTER_LEN: usize = 32; // TODO will be less on r32e, handle at somepoint

macro_rules! amo {
    ($frame:ident, $rs1:ident, $rs2:ident, $rd:ident, $operation:expr) => {
        let tmp = $frame[$rs1];
        let a = *(tmp as *const _);
        let b = $frame[$rs2];
        $frame[$rd] = a;
        *(tmp as *mut _) = $operation(a, b);
    };
}

/// Checks if the instruction is an atomic one.
#[inline(always)]
pub fn is_atomic_instruction(insn: usize) -> bool {
    (insn & 0b1111111) == 0b0101111
}

/// Takes the program counter address that triggered the exception and an array of
/// registers at point of exception with [`PLATFORM_REGISTER_LEN`] length.
/// Returns true if the instruction was atomic and was emulated, false otherwise.
///
/// # Safety
///
/// This function is supposed to be called right after the instruction caused an exception.
/// Thus, it assumes that the program counter is valid and points to a valid instruction.
/// It also assumes that all the user registers were correctly saved and sorted in a trap frame.
#[inline]
pub unsafe fn atomic_emulation(pc: usize, frame: &mut [usize; PLATFORM_REGISTER_LEN]) -> bool {
    static mut S_LR_ADDR: usize = 0;

    // SAFETY: program counter is valid and points to a valid instruction.
    let insn = unsafe { (pc as *const usize).read_unaligned() };
    if !is_atomic_instruction(insn) {
        return false;
    }

    let reg_mask = 0b11111;
    // destination register
    let rd = (insn >> 7) & reg_mask;
    // source 1 register
    let rs1 = (insn >> 15) & reg_mask;
    // source 2 register
    let rs2 = (insn >> 20) & reg_mask;

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
