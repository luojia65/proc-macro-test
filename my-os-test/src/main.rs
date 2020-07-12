#![no_std]
#![no_main]
#![feature(global_asm)]

use panic_halt as _;
use my_runtime_lib as _; // should be used as #[entry]

// if you don't need any boot page (i.e. run in bare mode),
// you don't use any of my_proc_macro's macros.
// or you need to pick one in your operating system code.

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
ffffffff80200014:       12000073                sfence.vma
ffffffff80200018:       00000097                auipc   ra,0x0
ffffffff8020001c:       0100b083                ld      ra,16(ra) # ffffffff80200028 <_start+0x28>
ffffffff80200020:       8082                    ret
ffffffff80200022:       00000013                nop
ffffffff80200026:       0001                    nop
ffffffff80200028:       0030                    addi    a2,sp,8
ffffffff8020002a:       8020                    0x8020
ffffffff8020002c:       ffff                    0xffff
ffffffff8020002e:       ffff                    0xffff

ffffffff80200030 <_abs_start>:
ffffffff80200030:       0000006f                j       ffffffff80200030 <_abs_start>

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
80400014:       12000073                sfence.vma
80400018:       00000097                auipc   ra,0x0
8040001c:       00c0a083                lw      ra,12(ra) # 80400024 <_start+0x24>
80400020:       8082                    ret
80400022:       0001                    nop
80400024:       0028                    addi    a0,sp,8
80400026:       8040                    0x8040

80400028 <_abs_start>:
80400028:       0000006f                j       80400028 <_abs_start>

Disassembly of section .rodata:

80401000 <_boot_page>:
        ...

*/
