use proc_macro::TokenStream;
use quote::quote;
// use syn::ItemConst;

#[proc_macro]
pub fn boot_page(item: TokenStream) -> TokenStream {
    println!("{:?}", item);
    use std::env;

    println!("{:?}", env::current_exe());

    quote!().into()
}
