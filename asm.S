#if __riscv_xlen == 64
# define STORE    sd
# define LOAD     ld
# define LOG_REGBYTES 3
#else
# define STORE    sw
# define LOAD     lw
# define LOG_REGBYTES 2
#endif
#define REGBYTES (1 << LOG_REGBYTES)

.section .trap, "ax"
.global _start_trap

_start_trap:
    addi sp, sp, -33*REGBYTES

    STORE zero, 0*REGBYTES(sp)
    STORE ra, 1*REGBYTES(sp)
    STORE sp, 2*REGBYTES(sp)
    STORE gp, 3*REGBYTES(sp)
    STORE tp, 4*REGBYTES(sp)
    STORE t0, 5*REGBYTES(sp)
    STORE t1, 6*REGBYTES(sp)
    STORE t2, 7*REGBYTES(sp)
    STORE fp, 8*REGBYTES(sp)
    STORE s1, 9*REGBYTES(sp)
    STORE a0, 10*REGBYTES(sp)
    STORE a1, 11*REGBYTES(sp)
    STORE a2, 12*REGBYTES(sp)
    STORE a3, 13*REGBYTES(sp)
    STORE a4, 14*REGBYTES(sp)
    STORE a5, 15*REGBYTES(sp)
    STORE a6, 16*REGBYTES(sp)
    STORE a7, 17*REGBYTES(sp)
    STORE s2, 18*REGBYTES(sp)
    STORE s3, 19*REGBYTES(sp)
    STORE s4, 20*REGBYTES(sp)
    STORE s5, 21*REGBYTES(sp)
    STORE s6, 22*REGBYTES(sp)
    STORE s7, 23*REGBYTES(sp)
    STORE s8, 24*REGBYTES(sp)
    STORE s9, 25*REGBYTES(sp)
    STORE s10, 26*REGBYTES(sp)
    STORE s11, 27*REGBYTES(sp)
    STORE t3, 28*REGBYTES(sp)
    STORE t4, 29*REGBYTES(sp)
    STORE t5, 30*REGBYTES(sp)
    STORE t6, 31*REGBYTES(sp)
    csrr t0, mepc
    STORE t0, 32*REGBYTES(sp)

    add a0, sp, zero
    jal ra, _start_trap_atomic_rust

    LOAD ra, 1*REGBYTES(sp)
    LOAD sp, 2*REGBYTES(sp)
    LOAD gp, 3*REGBYTES(sp)
    LOAD tp, 4*REGBYTES(sp)
    LOAD t0, 5*REGBYTES(sp)
    LOAD t1, 6*REGBYTES(sp)
    LOAD t2, 7*REGBYTES(sp)
    LOAD fp, 8*REGBYTES(sp)
    LOAD s1, 9*REGBYTES(sp)
    LOAD a0, 10*REGBYTES(sp)
    LOAD a1, 11*REGBYTES(sp)
    LOAD a2, 12*REGBYTES(sp)
    LOAD a3, 13*REGBYTES(sp)
    LOAD a4, 14*REGBYTES(sp)
    LOAD a5, 15*REGBYTES(sp)
    LOAD a6, 16*REGBYTES(sp)
    LOAD a7, 17*REGBYTES(sp)
    LOAD s2, 18*REGBYTES(sp)
    LOAD s3, 19*REGBYTES(sp)
    LOAD s4, 20*REGBYTES(sp)
    LOAD s5, 21*REGBYTES(sp)
    LOAD s6, 22*REGBYTES(sp)
    LOAD s7, 23*REGBYTES(sp)
    LOAD s8, 24*REGBYTES(sp)
    LOAD s9, 25*REGBYTES(sp)
    LOAD s10, 26*REGBYTES(sp)
    LOAD s11, 27*REGBYTES(sp)
    LOAD t3, 28*REGBYTES(sp)
    LOAD t4, 29*REGBYTES(sp)
    LOAD t5, 30*REGBYTES(sp)
    LOAD t6, 31*REGBYTES(sp)
    
    LOAD t0, 32*REGBYTES(sp)
    csrw mepc, t0

    addi sp, sp, 33*REGBYTES
    mret