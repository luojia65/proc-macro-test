use proc_macro::TokenStream;
use quote::quote;
// use syn::ItemConst;

#[proc_macro]
pub fn boot_page_sv39(item: TokenStream) -> TokenStream {
    println!("{:?}", item);

    quote!(
        #[repr(align(4096))]
        #[repr(C)]
        struct __BootPage([usize; 512]);
        #[export_name = "_boot_page"]
        static __BOOT_PAGE: __BootPage = __BootPage([0; 512]);
        extern { fn _abs_start() -> !; }
        global_asm!("
    .section .text.entry
    .globl _start
_start: 
    la t1, _boot_page
    li t0, 8 << 60
    or t0, t0, t1
    csrw satp, t0
    
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
    println!("{:?}", item);

    quote!(
        #[repr(align(4096))]
        #[repr(C)]
        struct __BootPage([usize; 512]);
        #[export_name = "_boot_page"]
        static __BOOT_PAGE: __BootPage = __BootPage([0; 512]);
        extern { fn _abs_start() -> !; }
        global_asm!("
    .section .text.entry
    .globl _start
_start: 
    la t1, _boot_page
    li t0, 9 << 60
    or t0, t0, t1
    csrw satp, t0
    
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
pub fn boot_page_sv32(item: TokenStream) -> TokenStream {
    println!("{:?}", item);

    quote!(
        #[repr(align(4096))]
        #[repr(C)]
        struct __BootPage([usize; 512]);
        #[export_name = "_boot_page"]
        static __BOOT_PAGE: __BootPage = __BootPage([0; 512]);
        extern { fn _abs_start() -> !; }
        global_asm!("
    .section .text.entry
    .globl _start
_start: 
    la t1, _boot_page
    li t0, 1 << 31
    or t0, t0, t1
    csrw satp, t0
    
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

#[proc_macro]
pub fn boot_page_bare(_item: TokenStream) -> TokenStream {
    quote!(
        global_asm!("
        .section .text.entry
        .globl _start
    _start: 
        j _abs_start
        ");
    ).into()
}
