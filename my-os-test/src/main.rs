#![no_std]
#![no_main]
#![feature(global_asm)]

use panic_halt as _;
use my_runtime_lib as _; // should be used as #[entry]

// #[cfg(target_pointer_width = "64")]
// my_proc_macro::boot_page_sv39! {
//     (0xffffffff_80000000 => 0x00000000_80000000, rwx);
//     (0xffffffff_00000000 => 0x00000000_00000000, rwx);
//     (0x00000000_80000000 => 0x00000000_80000000, rwx);
// }

// #[cfg(target_pointer_width = "32")]
// my_proc_macro::boot_page_sv32! {
//     (0x80000000 => 0x80000000, rwx);
//     (0x00000000 => 0x00000000, rwx);
// }

// PC-relocated generation in 64-bit
/*

Disassembly of section .text:

ffffffff80200000 <_start>:
ffffffff80200000:       00001317                auipc   t1,0x1
ffffffff80200004:       00030313                mv      t1,t1
ffffffff80200008:       52fd                    li      t0,-1
ffffffff8020000a:       12fe                    slli    t0,t0,0x3f
ffffffff8020000c:       0062e2b3                or      t0,t0,t1
ffffffff80200010:       18029073                csrw    satp,t0
ffffffff80200014:       00000097                auipc   ra,0x0
ffffffff80200018:       00c0b083                ld      ra,12(ra) # ffffffff80200020 <_start+0x20>
ffffffff8020001c:       8082                    ret
ffffffff8020001e:       0001                    nop
ffffffff80200020:       0028                    addi    a0,sp,8
ffffffff80200022:       8020                    0x8020
ffffffff80200024:       ffff                    0xffff
ffffffff80200026:       ffff                    0xffff

ffffffff80200028 <_abs_start>:
ffffffff80200028:       0000006f                j       ffffffff80200028 <_abs_start>

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
80400008:       800002b7                lui     t0,0x80000
8040000c:       0062e2b3                or      t0,t0,t1
80400010:       18029073                csrw    satp,t0
80400014:       00000097                auipc   ra,0x0
80400018:       00c0a083                lw      ra,12(ra) # 80400020 <_start+0x20>
8040001c:       8082                    ret
8040001e:       0001                    nop
80400020:       0024                    addi    s1,sp,8
80400022:       8040                    0x8040

80400024 <_abs_start>:
80400024:       0000006f                j       80400024 <_abs_start>

Disassembly of section .rodata:

80401000 <_boot_page>:
        ...

*/
