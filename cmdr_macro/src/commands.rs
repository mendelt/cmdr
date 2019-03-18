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
    let method_name = &method.sig.ident;
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
            command: method_name.to_string(),
            method: method_name.to_owned(),
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
                Some(string.value().trim().to_owned() + "\n")
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
#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_ignore_method_without_cmd_attribute() {
        let source = syn::parse_str(
            r###"
            fn method() {}
        "###,
        )
        .unwrap();

        assert_eq!(parse_cmd_attribute(&source), None);
    }

    #[test]
    fn should_parse_plain_cmd_attribute() {
        let source = syn::parse_str(
            r###"
            #[cmd]
            fn method() {}
        "###,
        )
        .unwrap();

        let parsed = parse_cmd_attribute(&source).unwrap();

        assert_eq!(parsed.command, "method".to_string());
        assert_eq!(parsed.method.to_string(), "method".to_string());
    }

    #[test]
    fn should_parse_outer_doc_string_as_help_text() {
        let source = syn::parse_str(
            r###"
            #[cmd]
            ///Help text
            fn method() {}
        "###,
        )
        .unwrap();

        let parsed = parse_cmd_attribute(&source).unwrap();
        assert_eq!(parsed.help, "Help text\n".to_string());
    }

    #[test]
    fn should_parse_inner_doc_string_as_help_text() {
        let source = syn::parse_str(
            r###"
            #[cmd]
            fn method() {
                //!Help text
            }
        "###,
        )
        .unwrap();

        let parsed = parse_cmd_attribute(&source).unwrap();

        assert_eq!(parsed.help, "Help text\n".to_string());
    }

    #[test]
    fn should_strip_help_text_spaces() {
        let source = syn::parse_str(
            r###"
            #[cmd]
            ///     Help text
            fn method() {}
        "###,
        )
        .unwrap();

        let parsed = parse_cmd_attribute(&source).unwrap();

        assert_eq!(parsed.help, "Help text\n".to_string());
    }

    #[test]
    fn should_parse_multi_line_help_text() {
        let source = syn::parse_str(
            r###"
            #[cmd]
            /// Multi line
            /// help text
            fn method() {}
        "###,
        )
        .unwrap();

        let parsed = parse_cmd_attribute(&source).unwrap();

        assert_eq!(parsed.help, "Multi line\nhelp text\n".to_string());
    }
}
