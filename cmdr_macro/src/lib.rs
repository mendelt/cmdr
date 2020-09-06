//! **Cmdr_macro contains macro's for use with the cmdr crate.**
//!
//! The cmdr_macro crate should not be used separately. The cmdr crate can
//! be found here:
//!
//! [Crates.io](https://crates.io/crates/cmdr)

// Turn on warnings for some lints
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_import_braces,
    unused_qualifications
)]

extern crate proc_macro;
extern crate proc_macro2;

mod commands;
mod overrides;
mod parsing;

use crate::commands::format_commands;
use crate::overrides::format_overrides;
use crate::parsing::parse_self_type;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemImpl};

/// Implements the cmdr::Scope trait on any impl block.
///
/// The macro can be used to annotate any plain impl block it will then generate an additional
/// impl block to implement Scope for the same type.
///
/// Right now it will search the impl block for methods starting with do_ and call them in a
/// generated Scope::command method when the right command is received.
#[proc_macro_attribute]
pub fn cmdr(meta_stream: TokenStream, code_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(code_stream as ItemImpl);
    let meta = parse_macro_input!(meta_stream as AttributeArgs);

    let self_type = parse_self_type(&input).unwrap();
    let self_generics = &input.generics;
    let self_where = &self_generics.where_clause;

    let commands = format_commands(&input, &meta);
    let overrides = format_overrides(&input, &self_type);

    TokenStream::from(quote!(
        #input

        impl#self_generics cmdr::Scope for #self_type #self_where {
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
