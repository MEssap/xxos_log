use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Logger)]
pub fn derive(input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    quote!(
        impl WriteLog for #st_name {
            fn print(&self, log_content: core::fmt::Arguments) {
                println!("{}", log_content)
            }
        }
    ).into()
}