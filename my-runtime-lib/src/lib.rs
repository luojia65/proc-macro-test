#![no_std]
#![feature(global_asm)]

extern {
    static _boot_page: [usize; 0];
    // static _boot_satp_mode: usize;
}

// global_asm!(r#"
//     .equ _boot_satp_mode, 0
// "#);

#[cfg(target_pointer_width = "64")]
global_asm!(r#"
    .section .text.entry
    .globl _start
_start: 
    .if _boot_satp_mode != 0
    la t1, _boot_page
    li t0, _boot_satp_mode
    slli t0, t0, 60
    or t0, t0, t1
    csrw satp, t0
    .endif
    
    .option push
    .option norelax
1:
    auipc ra, %pcrel_hi(1f)
    ld ra, %pcrel_lo(1b)(ra)
    jr ra
    .align  3
1:
    .dword _abs_start
.option pop

_abs_start:
    j _abs_start
"#);

#[cfg(target_pointer_width = "32")]
global_asm!(r#"
    .section .text.entry
    .globl _start
_start: 
    .if _boot_satp_mode != 0
    la t1, _boot_page
    li t0, _boot_satp_mode
    slli t0, t0, 30
    or t0, t0, t1
    csrw satp, t0
    .endif

    .option push
    .option norelax
1:
    auipc ra, %hi(1f)
    lw ra, %lo(1b)(ra)
    jr ra
    .align  2
1:
    .word _abs_start
.option pop

_abs_start:
    j _abs_start
"#);
