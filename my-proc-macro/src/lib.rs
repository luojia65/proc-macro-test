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
        global_asm!(".equ _boot_satp_mode, 8");
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
        global_asm!(".equ _boot_satp_mode, 9");
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
        global_asm!(".equ _boot_satp_mode, 1");
    ).into()
}

#[proc_macro]
pub fn boot_page_bare(item: TokenStream) -> TokenStream {
    println!("{:?}", item);

    quote!(
        global_asm!(".equ _boot_satp_mode, 0");
    ).into()
}
