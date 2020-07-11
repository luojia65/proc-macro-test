#![no_std]
#![feature(global_asm)]

global_asm!("
    .section .text.entry
    .globl _abs_start
_abs_start:
    j _abs_start
");
