use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, ImplItem, ItemImpl, Meta, MetaList, NestedMeta, TypePath};

pub fn format_commands(input: &ItemImpl, self_type: &TypePath) -> TokenStream {
    let command_methods = get_methods(&input);
    let scope_help = parse_help_text(&input.attrs);
    quote!(
        fn commands() -> ScopeDescription<#self_type> {
            ScopeDescription::new(
                Some(#scope_help.to_string()),
                vec![#(#command_methods)*]
            )
        }
    )
}

fn get_methods(input: &ItemImpl) -> Vec<(CmdMeta)> {
    input
        .items
        .iter()
        .filter_map(parse_cmd_attributes)
        .collect()
}

fn parse_cmd_attributes(item: &ImplItem) -> Option<CmdMeta> {
    if let ImplItem::Method(method) = item {
        let attributes = &method.attrs;

        let cmd_attributes: Vec<Meta> = attributes
            .iter()
            .filter_map(Attribute::interpret_meta)
            .filter(|meta| meta.name() == "cmd")
            .collect();

        if !cmd_attributes.is_empty() {
            let help_text = parse_help_text(attributes);

            let method_ident = method.sig.ident.to_owned();
            let mut command_name = method_ident.to_string();

            // Parse cmd field
            for meta in cmd_attributes {
                // Parse command name if it is different from method name
                // #[cmd(command_name)]
                if let Meta::List(MetaList { nested, .. }) = meta {
                    if nested.len() == 1 {
                        if let NestedMeta::Meta(Meta::Word(ident)) = &nested[0] {
                            command_name = ident.to_string()
                        }
                    }
                }
            }

            Some(CmdMeta {
                command: command_name,
                method: method_ident,
                help: help_text,
            })
        } else {
            // Method has no cmd attribute so is not a command
            None
        }
    } else {
        // Not a method
        None
    }
}

/// Parse documentation from attributes
fn parse_help_text(attrs: &Vec<Attribute>) -> String {
    attrs
        .iter()
        .filter_map(Attribute::interpret_meta)
        .filter(|meta| meta.name() == "doc")
        .filter_map(parse_doc_string)
        .collect()
}

fn parse_doc_string(meta: Meta) -> Option<String> {
    if let Meta::NameValue(name_val) = meta {
        if let syn::Lit::Str(string) = name_val.lit {
            Some(string.value().trim().to_owned() + "\n")
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
            ScopeCmdDescription::new(
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

        assert_eq!(parse_cmd_attributes(&source), None);
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

        let parsed = parse_cmd_attributes(&source).unwrap();

        assert_eq!(parsed.command, "method".to_string());
        assert_eq!(parsed.method.to_string(), "method".to_string());
    }

    #[test]
    fn should_parse_command_name() {
        let source = syn::parse_str(
            r###"
                #[cmd(command)]
                fn method() {}
            "###,
        )
        .unwrap();

        let parsed = parse_cmd_attributes(&source).unwrap();
        assert_eq!(parsed.command, "command".to_string());
        assert_eq!(parsed.method.to_string(), "method".to_string());
    }

    #[test]
    fn should_parse_name_from_multiple_cmd_attributes() {
        let source = syn::parse_str(
            r###"
                #[cmd]
                #[cmd(command)]
                fn method() {}
           "###,
        )
        .unwrap();

        let parsed = parse_cmd_attributes(&source).unwrap();
        assert_eq!(parsed.command, "command".to_string());
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

        let parsed = parse_cmd_attributes(&source).unwrap();
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

        let parsed = parse_cmd_attributes(&source).unwrap();

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

        let parsed = parse_cmd_attributes(&source).unwrap();

        assert_eq!(parsed.help, "Help text\n".to_string());
    }

    #[test]
    fn should_parse_multi_line_help_text() {
        let source = syn::parse_str(
            r###"
            #[cmd(name)]
            /// Multi line
            /// help text
            fn method() {}
        "###,
        )
        .unwrap();

        let parsed = parse_cmd_attributes(&source).unwrap();

        assert_eq!(parsed.help, "Multi line\nhelp text\n".to_string());
    }
}
