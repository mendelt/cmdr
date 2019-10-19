use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    Attribute, ImplItem, ItemImpl, Lit, Meta, MetaList, MetaNameValue, NestedMeta, TypePath,
};

pub fn format_commands(input: &ItemImpl, self_type: &TypePath) -> TokenStream {
    let command_methods = get_methods(&input);
    let scope_help = parse_help_text(&input.attrs);
    quote!(
        fn commands() -> ScopeDescription<#self_type> {
            ScopeDescription::new(
                Some(#scope_help.to_string()),
                None,
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
            .map(Attribute::parse_meta)
            .filter_map(Result::ok)
            .filter(|meta| meta.path().is_ident("cmd"))
            .collect();

        if !cmd_attributes.is_empty() {
            let method_ident = method.sig.ident.to_owned();

            let mut help_text = parse_help_text(attributes);
            let mut command_name = method_ident.to_string();
            let mut aliasses = Vec::new();

            // Parse cmd fields
            for meta in cmd_attributes {
                // Parse command name if it is different from method name
                // #[cmd(command_name)]
                if let Meta::List(MetaList { nested, .. }) = meta {
                    for nested_val in nested {
                        match nested_val {
                            NestedMeta::Meta(Meta::Path(ref path)) => {
                                if let Some(ident) = path.get_ident() {
                                    command_name = ident.to_string()
                                }
                            }
                            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                path,
                                lit: Lit::Str(lit),
                                ..
                            })) => {
                                if path.is_ident("name") {
                                    command_name = lit.value();
                                } else if path.is_ident("help") {
                                    help_text = lit.value();
                                }
                            }
                            NestedMeta::Meta(Meta::List(ref alias_list))
                                if alias_list.path.is_ident("alias") =>
                            {
                                for alias_item in &alias_list.nested {
                                    if let NestedMeta::Meta(Meta::Path(ref alias_path)) = alias_item
                                    {
                                        if let Some(alias_ident) = alias_path.get_ident() {
                                            aliasses.push(alias_ident.to_string());
                                        }
                                    }
                                    if let NestedMeta::Lit(Lit::Str(alias_lit)) = alias_item {
                                        aliasses.push(alias_lit.value());
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            Some(CmdMeta {
                command: command_name,
                method: method_ident,
                alias: aliasses,
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
        .map(Attribute::parse_meta)
        .filter_map(Result::ok)
        .filter(|meta| meta.path().is_ident("doc"))
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
    alias: Vec<String>,
    help: String,
}

impl ToTokens for CmdMeta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let command = &self.command;
        let method = &self.method;
        let help_text = &self.help;
        let alias_list: Vec<TokenStream> = self
            .alias
            .iter()
            .map(|alias| quote!(#alias.to_string()))
            .collect();
        let alias_quote = quote!(vec![#(#alias_list),*]);

        tokens.extend(quote!(
            ScopeCmdDescription::new(
                #command.to_string(),
                Box::new(|scope, cmd_line| scope.#method(&cmd_line.args)),
                #alias_quote,
                Some(#help_text.to_string()),
            ),
        ))
    }
}

#[cfg(test)]
mod when_parsing_function_cmd_attributes {
    use super::*;

    #[test]
    fn should_ignore_method_without_cmd_attribute() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                fn method() {}
                "###,
            )
            .unwrap(),
        );

        assert_eq!(parsed, None);
    }

    #[test]
    fn should_parse_plain_cmd_attribute() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd]
                fn method() {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.command, "method".to_string());
        assert_eq!(parsed.method.to_string(), "method".to_string());
    }

    #[test]
    fn should_parse_command_name() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd(command)]
                fn method() {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.command, "command".to_string());
        assert_eq!(parsed.method.to_string(), "method".to_string());
    }

    #[test]
    fn should_parse_named_command_name() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd(name="command")]
                fn method() {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.command, "command".to_string());
        assert_eq!(parsed.method.to_string(), "method".to_string());
    }

    #[test]
    fn should_parse_name_from_multiple_cmd_attributes() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd]
                #[cmd(command)]
                fn method() {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.command, "command".to_string());
        assert_eq!(parsed.method.to_string(), "method".to_string());
    }

    #[test]
    fn should_parse_outer_doc_string_as_help_text() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd]
                ///Help text
                fn method() {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();
        // TODO: Remove trailing newline
        assert_eq!(parsed.help, "Help text\n".to_string());
    }

    #[test]
    fn should_parse_inner_doc_string_as_help_text() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd]
                fn method() {
                    //!Help text
                }
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.help, "Help text\n".to_string());
    }

    #[test]
    fn should_strip_help_text_spaces() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd]
                ///     Help text
                fn method() {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.help, "Help text\n".to_string());
    }

    #[test]
    fn should_ignore_docstring_if_cmd_attribute_help_available() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd(name, help="Help text from the cmd attribute")]
                /// This is a docstring, not help text
                fn method() {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.help, "Help text from the cmd attribute".to_string())
    }

    #[test]
    fn should_parse_multiline_help_from_cmd_attribute() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd(name, help="Multiline help text\nFrom the cmd attribute")]
                fn method() {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(
            parsed.help,
            "Multiline help text\nFrom the cmd attribute".to_string()
        )
    }

    // TODO: implement this
    //
    //    #[test]
    //    fn should_set_missing_help_text_to_none() {
    //        let parsed = parse_cmd_attributes(
    //            &syn::parse_str(
    //                r###"
    //                #[cmd(name)]
    //                fn method() {}
    //            "###,
    //            )
    //                .unwrap(),
    //        )
    //            .unwrap();
    //        assert_eq!(parsed.help, None)
    //    }

    #[test]
    fn should_parse_alias_from_cmd_attribute() {
        let parsed = parse_cmd_attributes(
            &syn::parse_str(
                r###"
                #[cmd(name, alias("one", "two", three))]
                fn method() {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.alias, vec!["one", "two", "three"]);
    }
}
