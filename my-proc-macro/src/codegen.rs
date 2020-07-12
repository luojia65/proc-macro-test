use proc_macro2::TokenStream as TokenStream;
use quote::quote;
use proc_macro2::Literal;
use crate::syntax::{Mode, EntryConfig};

pub fn boot_page_content(entry_config: &EntryConfig, mode: Mode) -> TokenStream {
    match mode {
        // in Sv32, virtual page number contain 10 bits
        Mode::Sv32 => {
            let pte = (0..1024)
                .map(|idx| entry_config[idx])
                .map(|val| Literal::usize_unsuffixed(val));
            quote!( #( #pte , )* )
        }
        // in Sv39 and Sv48, virtual page number contain 9 bits
        Mode::Sv39 | Mode::Sv48 => {
            let pte = (0..512)
                .map(|idx| entry_config[idx])
                .map(|val| Literal::usize_unsuffixed(val));
            quote!( #( #pte , )* )
        }
    }
}

/*

    Generated like:

    0, 0, 0, /* and all 512 usize page table entries */, 0,

*/
