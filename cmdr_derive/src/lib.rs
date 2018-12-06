extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;


#[proc_macro_derive(ContextMacro)]
pub fn context_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let name = &ast.ident;
    let gen = quote! {
        impl ContextMacro for #name {
            fn do_stuff(&self) {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(gen)
}
