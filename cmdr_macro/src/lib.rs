extern crate proc_macro;
extern crate proc_macro2;

use self::proc_macro::TokenStream;
use self::proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, ItemImpl};
use quote::quote;
use syn::*;


/// Macro that implements the cmdr::Scope trait for you.
///
/// The macro can be used to annotate any plain impl block it will then generate an additional
/// impl block to implement Scope for the same type.
///
/// Right now it will search the impl block for methods starting with do_ and call them in a
/// generated Scope::command method when the right command is received.
#[proc_macro_attribute]
pub fn cmdr(_meta: TokenStream, code: TokenStream) -> TokenStream {
    let input = parse_macro_input!(code as ItemImpl);
    let command_matches = format_command_match(&get_methods(&input));

    if let Type::Path(self_type) = &*input.self_ty {

        let output = TokenStream::from(quote!(
            #input

            impl cmdr::Scope for #self_type {
                fn command(&mut self, line: Line) -> CommandResult {
                    match line {
                        Line::Empty => self.empty(),
                        #(#command_matches),*,
                        _ => self.default(line)
                    }
                }
            }
        ));

        output
    }
    else {
        panic!("Unable to parse impl type")
    }
}


fn format_command_match(methods: &Vec<(Ident, String)>) -> Vec<TokenStream2> {
    let mut result: Vec<TokenStream2> = Vec::new();

    for (method, name) in methods {
        result.push(quote!(Line::Command(#name, args) => self.#method(args)));
    }

    result
}


fn get_methods(input: &ItemImpl) -> Vec<(Ident, String)> {
    let mut result: Vec<(Ident, String)> = Vec::new();

    for item in &input.items {
        if let ImplItem::Method(method) = item {
            let ident = &method.sig.ident;
            let name = ident.to_string();

            if name.starts_with("do_") {
                result.push((ident.clone(), name[3..].to_owned()))
            }
        }
    }

    result
}
