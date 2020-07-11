#![no_std]
#![no_main]
#![feature(global_asm)]

use panic_halt as _;

#[cfg(target_pointer_width = "64")]
my_proc_macro::boot_page_sv39! {
    (0xffffffff_80000000 => 0x00000000_80000000, rwx);
    (0xffffffff_00000000 => 0x00000000_00000000, rwx);
    (0x00000000_80000000 => 0x00000000_80000000, rwx);
}

#[cfg(target_pointer_width = "32")]
my_proc_macro::boot_page_sv32! {
    (0x80000000 => 0x80000000, rwx);
    (0x00000000 => 0x00000000, rwx);
}

use my_runtime_lib as _;

// PC-relocated generation in 64-bit
/*

Disassembly of section .text:

ffffffff80200000 <_start>:
ffffffff80200000:       00001317                auipc   t1,0x1
ffffffff80200004:       00030313                mv      t1,t1
ffffffff80200008:       42a1                    li      t0,8
ffffffff8020000a:       03c41293                slli    t0,s0,0x3c
ffffffff8020000e:       0062e2b3                or      t0,t0,t1
ffffffff80200012:       18029073                csrw    satp,t0

Disassembly of section .rodata:

ffffffff80201000 <_boot_page>:
        ...

*/

// PC-relocated generation in 32-bit
/*

Disassembly of section .text:

80400000 <_start>:
80400000:       00001317                auipc   t1,0x1
80400004:       00030313                mv      t1,t1
80400008:       4285                    li      t0,1
8040000a:       02fa                    slli    t0,t0,0x1e
8040000c:       0062e2b3                or      t0,t0,t1
80400010:       18029073                csrw    satp,t0

Disassembly of section .rodata:

80401000 <_boot_page>:
        ...

*/
