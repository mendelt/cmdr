extern crate proc_macro;
extern crate proc_macro2;

mod overrides;

use self::proc_macro::TokenStream;
use self::proc_macro2::TokenStream as TokenStream2;
use crate::overrides::format_overrides;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, ImplItem, ItemImpl, Type};
use syn::{ImplItemMethod, Meta};

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

    if let Type::Path(self_type) = &*input.self_ty {
        let command_methods = get_methods(&input);
        let overrides = format_overrides(&input);

        TokenStream::from(quote!(
            #input

            impl cmdr::Scope for #self_type {
                fn commands() -> CmdMethodList<#self_type> {
                    CmdMethodList::new(vec![
                        #(#command_methods)*
                    ])
                }

                #overrides
            }
        ))
    } else {
        panic!("Unable to parse impl type")
    }
}

/// Use cmd attribute to mark methods as cmdr commands.
#[proc_macro_attribute]
pub fn cmd(_meta: TokenStream, code: TokenStream) -> TokenStream {
    code
}

fn get_methods(input: &ItemImpl) -> Vec<(CmdMeta)> {
    let mut result = Vec::new();

    for item in &input.items {
        if let ImplItem::Method(method) = item {
            if let Some(metadata) = parse_cmd_attribute(method) {
                result.push(metadata)
            }
        }
    }

    result
}

fn parse_cmd_attribute(method: &ImplItemMethod) -> Option<CmdMeta> {
    let methodname = &method.sig.ident;
    let cmd_attr = method
        .attrs
        .iter()
        .map(|arg| arg.parse_meta())
        .filter_map(Result::ok)
        .filter(|meta| meta.name() == "cmd")
        .next();

    let help_text = parse_help_text(method);

    match cmd_attr {
        Some(_attr) => Some(CmdMeta {
            command: methodname.to_string(),
            method: methodname.to_owned(),
            help: help_text,
        }),
        None => None,
    }
}

fn parse_help_text(method: &ImplItemMethod) -> String {
    let doc_attrs: String = method
        .attrs
        .iter()
        .map(|arg| arg.parse_meta())
        .filter_map(Result::ok)
        .filter_map(parse_doc_string)
        .collect();

    doc_attrs
}

fn parse_doc_string(meta: Meta) -> Option<String> {
    if meta.name() == "doc" {
        if let Meta::NameValue(name_val) = meta {
            if let syn::Lit::Str(string) = name_val.lit {
                Some(string.value() + "\n")
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// Contains all metadata for a command
struct CmdMeta {
    command: String,
    method: Ident,
    help: String,
}

impl ToTokens for CmdMeta {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let command = &self.command;
        let method = &self.method;
        let help_text = &self.help;

        tokens.extend(quote!(
            CmdMethod::new(
                #command.to_string(),
                Box::new(|scope, cmd_line| scope.#method(&cmd_line.args)),
                Some(#help_text.to_string()),
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
