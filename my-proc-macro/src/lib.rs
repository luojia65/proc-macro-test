mod codegen;
mod syntax;

use proc_macro::TokenStream;
use quote::quote;
use crate::syntax::Mode;

#[proc_macro]
pub fn boot_page_sv39(item: TokenStream) -> TokenStream {
    
    let entry_config = match syntax::parse(item.into(), Mode::Sv39) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    let boot_page_content = codegen::boot_page_content(&entry_config, Mode::Sv39);

    quote!(
        #[repr(align(4096))]
        #[repr(C)]
        struct __BootPage([usize; 512]);
        #[export_name = "_boot_page"]
        static __BOOT_PAGE: __BootPage = __BootPage([ #boot_page_content ]);
        extern { fn _abs_start() -> !; }
        global_asm!("
    .section .text.entry
    .globl _start
_start: 
    la t1, _boot_page
    srli t1, t1, 12
    li t0, 8 << 60
    or t0, t0, t1
    csrw satp, t0
    sfence.vma
    
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
        ");
    ).into()
}

#[proc_macro]
pub fn boot_page_sv48(item: TokenStream) -> TokenStream {
    
    let entry_config = match syntax::parse(item.into(), Mode::Sv48) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    let boot_page_content = codegen::boot_page_content(&entry_config, Mode::Sv48);

    quote!(
        #[repr(align(4096))]
        #[repr(C)]
        struct __BootPage([usize; 512]);
        #[export_name = "_boot_page"]
        static __BOOT_PAGE: __BootPage = __BootPage([ #boot_page_content ]);
        extern { fn _abs_start() -> !; }
        global_asm!("
    .section .text.entry
    .globl _start
_start: 
    la t1, _boot_page
    srli t1, t1, 12
    li t0, 9 << 60
    or t0, t0, t1
    csrw satp, t0
    sfence.vma
    
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
        ");
    ).into()
}

// There should be sv57, sv64 here in the future

#[proc_macro]
pub fn boot_page_sv32(item: TokenStream) -> TokenStream {
    
    let entry_config = match syntax::parse(item.into(), Mode::Sv32) {
        Err(e) => return e.to_compile_error().into(),
        Ok(x) => x,
    };

    let boot_page_content = codegen::boot_page_content(&entry_config, Mode::Sv32);

    quote!(
        #[repr(align(4096))]
        #[repr(C)]
        struct __BootPage([usize; 1024]);
        #[export_name = "_boot_page"]
        static __BOOT_PAGE: __BootPage = __BootPage([ #boot_page_content ]);
        extern { fn _abs_start() -> !; }
        global_asm!("
    .section .text.entry
    .globl _start
_start: 
    la t1, _boot_page
    srli t1, t1, 12
    li t0, 1 << 31
    or t0, t0, t1
    csrw satp, t0
    sfence.vma
    
    .option push
    .option norelax
1:
    auipc ra, %pcrel_hi(1f)
    lw ra, %pcrel_lo(1b)(ra)
    jr ra
    .align  2
1:
    .word _abs_start
.option pop
        ");
    ).into()
}

// if you need boot_page_bare, you don't include any macro in this crate.
