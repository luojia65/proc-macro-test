use proc_macro2::TokenStream as TokenStream;
use quote::quote;
use crate::syntax::EntryConfig;

pub fn boot_page_content(entry_config: &EntryConfig) -> TokenStream {
    let pte = (0..512)
        .map(|idx| entry_config[idx])
        .map(syn::Index::from);
    quote!(
        #( #pte , )*
    )
}

/*

    Generated like:

    #[export_name = "_boot_page"]
    static __BOOT_PAGE: __BootPage = __BootPage([
        0, 0, 0, /* and all 512 usize page table entries */
    ]);

*/
