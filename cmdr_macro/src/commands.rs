use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{ImplItem, ImplItemMethod, ItemImpl, Meta, TypePath};

pub fn format_commands(input: &ItemImpl, self_type: &TypePath) -> TokenStream {
    let command_methods = get_methods(&input);

    quote!(
        fn commands() -> CmdMethodList<#self_type> {
            CmdMethodList::new(vec![
                #(#command_methods)*
            ])
        }
    )
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
