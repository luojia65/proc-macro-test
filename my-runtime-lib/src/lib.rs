#![no_std]
#![feature(global_asm)]

// when run in bare mode, there is no _start symbol in the output file;
// there is _abs_start only in the beginning of the runtime entry address

global_asm!("
    .section .text.entry
    .globl _start
    .weak _start
_start:
    .globl _abs_start
_abs_start:
    j _abs_start
");
