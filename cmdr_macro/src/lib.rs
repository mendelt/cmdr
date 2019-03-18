extern crate proc_macro;
extern crate proc_macro2;

mod commands;
mod overrides;
mod util;

use self::proc_macro::TokenStream;
use crate::commands::format_commands;
use crate::overrides::format_overrides;
use crate::util::parse_self_type;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

/// Implements the cmdr::Scope trait on any impl block.
///
/// The macro can be used to annotate any plain impl block it will then generate an additional
/// impl block to implement Scope for the same type.
///
/// Right now it will search the impl block for methods starting with do_ and call them in a
/// generated Scope::command method when the right command is received.
#[proc_macro_attribute]
pub fn cmdr(_meta: TokenStream, code: TokenStream) -> TokenStream {
    let input: ItemImpl = parse_macro_input!(code);
    let self_type = parse_self_type(&input).unwrap();

    let commands = format_commands(&input, &self_type);
    let overrides = format_overrides(&input, &self_type);

    TokenStream::from(quote!(
        #input

        impl cmdr::Scope for #self_type {
            #commands
            #overrides
        }
    ))
}

/// Use cmd attribute to mark methods as cmdr commands.
#[proc_macro_attribute]
pub fn cmd(_meta: TokenStream, code: TokenStream) -> TokenStream {
    code
}
