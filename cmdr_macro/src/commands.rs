use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    Attribute, AttributeArgs, ImplItem, ImplItemMethod, ItemImpl, Lit, Meta, MetaList,
    MetaNameValue, NestedMeta, ReturnType, Type,
};

pub(crate) fn format_commands(input: &ItemImpl, meta: &AttributeArgs) -> TokenStream {
    let (help_text, help_command) = parse_cmdr_attributes(meta);
    let doc_help_text = parse_help_text(&input.attrs);

    let mut command_methods = parse_commands(&input);

    if let Some(command) = help_command {
        command_methods.insert(
            0,
            CmdAttributes {
                command: command.clone(),
                method: Ident::new("help", Span::call_site()),
                alias: vec![],
                help: None,
            },
        )
    }
    let command_calls: Vec<_> = command_methods.iter().map(CmdAttributes::to_call).collect();

    let quoted_help = quote_string_option(&help_text.or(doc_help_text));

    quote!(
        fn commands(&self) -> ScopeDescription {
            ScopeDescription::new(
                #quoted_help,
                vec![#(#command_methods)*]
            )
        }

        fn run_command(&mut self, command: &ScopeCmdDescription, args: &[String], writer: &mut dyn LineWriter) -> CommandResult {
            match command.name() {
                #(#command_calls)*
                _ => Err(Error::InvalidCommand(command.name().to_string()))
            }
        }
    )
}

/// Parses the help text and help command from the cmdr attribute
fn parse_cmdr_attributes(meta: &AttributeArgs) -> (Option<String>, Option<String>) {
    let mut help_text = None;
    let mut help_command = Some("help".to_string());

    for meta_item in meta {
        match meta_item {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit),
                ..
            })) => {
                if path.is_ident("help") {
                    help_text = Some(lit.value());
                }
                if path.is_ident("help_command") & help_command.is_some() {
                    help_command = Some(lit.value());
                }
            }
            NestedMeta::Meta(Meta::Path(path)) => {
                if path.is_ident("nohelp") | path.is_ident("no_help") {
                    help_command = None
                }
            }
            _ => (),
        }
    }

    (help_text, help_command)
}

fn quote_string_option(value: &Option<String>) -> TokenStream {
    match value {
        Some(text) => quote!(Some(#text.to_string())),
        None => quote!(None),
    }
}

/// Parse attributes for several commands
fn parse_commands(input: &ItemImpl) -> Vec<CmdAttributes> {
    input
        .items
        .iter()
        .filter_map(parse_cmd_attributes)
        .collect()
}

/// Check if this method has the right signature to be a cmd, panics if it doesnt
fn check_cmd_signature(method: &ImplItemMethod) {
    let method_ident = method.sig.ident.to_owned();

    // Get method parameters and check that they are the right type
    let ins: Vec<_> = method.sig.inputs.iter().collect();

    if ins.len() != 2 {
        panic!(format!(
            "Invalid signature for command {}, expected '&mut self, args &[String]'", method_ident
        ));
    }

    // Check method return type
    if let ReturnType::Type(_, tpy) = method.sig.output.clone() {
        if let Type::Path(tpy2) = tpy.as_ref() {
            if !tpy2
                .path
                .is_ident(&Ident::new("CommandResult", Span::call_site()))
            {
                panic!(format!(
                    "Wrong return type for command {}, should be CommandReult",
                    method_ident
                ));
            }
        }
    }
}

/// Parse attributes for a single command
fn parse_cmd_attributes(item: &ImplItem) -> Option<CmdAttributes> {
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

            check_cmd_signature(method);

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
                                    help_text = Some(lit.value());
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

            Some(CmdAttributes {
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
fn parse_help_text(attrs: &Vec<Attribute>) -> Option<String> {
    let mut help_lines = attrs
        .iter()
        .map(Attribute::parse_meta)
        .filter_map(Result::ok)
        .filter(|meta| meta.path().is_ident("doc"))
        .filter_map(parse_doc_string)
        .peekable();

    if help_lines.peek().is_some() {
        Some(help_lines.join("\n"))
    } else {
        None
    }
}

fn parse_doc_string(meta: Meta) -> Option<String> {
    if let Meta::NameValue(name_val) = meta {
        if let syn::Lit::Str(string) = name_val.lit {
            Some(string.value().trim().to_owned())
        } else {
            None
        }
    } else {
        None
    }
}

/// Contains all metadata for a command
#[derive(Debug, PartialEq)]
struct CmdAttributes {
    command: String,
    method: Ident,
    alias: Vec<String>,
    help: Option<String>,
}

impl CmdAttributes {
    pub(crate) fn to_call(&self) -> CmdCall {
        CmdCall {
            command: self.command.clone(),
            method: self.method.clone(),
        }
    }
}

impl ToTokens for CmdAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let command = &self.command;
        let help_text = quote_string_option(&self.help);
        let alias_list: Vec<TokenStream> = self
            .alias
            .iter()
            .map(|alias| quote!(#alias.to_string()))
            .collect();
        let alias_quote = quote!(vec![#(#alias_list),*]);

        tokens.extend(quote!(
            ScopeCmdDescription::new(
                #command.to_string(),
                #alias_quote,
                #help_text,
            ),
        ))
    }
}

/// Contains all metadata for generating a command call
#[derive(Debug, PartialEq)]
struct CmdCall {
    command: String,
    method: Ident,
}

impl ToTokens for CmdCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let command = &self.command;
        let method = &self.method;

        tokens.extend(quote!(
            #command => self.#method(args),
        ));
    }
}

#[cfg(test)]
mod when_parsing_function_cmd_attributes {
    use super::*;
    use syn::parse_str;

    #[test]
    fn should_ignore_method_without_cmd_attribute() {
        let parsed = parse_cmd_attributes(
            &parse_str(
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
            &parse_str(
                r###"
                #[cmd]
                fn method(&self, args: &[String]) {}
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
            &parse_str(
                r###"
                #[cmd(command)]
                fn method(&self, args: &[String]) {}
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
            &parse_str(
                r###"
                #[cmd(name="command")]
                fn method(&self, args: &[String]) {}
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
            &parse_str(
                r###"
                #[cmd]
                #[cmd(command)]
                fn method(&self, args: &[String]) {}
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
            &parse_str(
                r###"
                #[cmd]
                ///Help text
                fn method(&self, args: &[String]) {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(parsed.help.unwrap(), "Help text".to_string());
    }

    #[test]
    fn should_parse_inner_doc_string_as_help_text() {
        let parsed = parse_cmd_attributes(
            &parse_str(
                r###"
                #[cmd]
                fn method(&self, args: &[String]) {
                    //!Help text
                }
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.help.unwrap(), "Help text".to_string());
    }

    #[test]
    fn should_strip_help_text_spaces() {
        let parsed = parse_cmd_attributes(
            &parse_str(
                r###"
                #[cmd]
                ///     Help text
                fn method(&self, args: &[String]) {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.help.unwrap(), "Help text".to_string());
    }

    #[test]
    fn should_ignore_docstring_if_cmd_attribute_help_available() {
        let parsed = parse_cmd_attributes(
            &parse_str(
                r###"
                #[cmd(name, help="Help text from the cmd attribute")]
                /// This is a docstring, not help text
                fn method(&self, args: &[String]) {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(
            parsed.help.unwrap(),
            "Help text from the cmd attribute".to_string()
        )
    }

    #[test]
    fn should_parse_multiline_help_from_cmd_attribute() {
        let parsed = parse_cmd_attributes(
            &parse_str(
                r###"
                #[cmd(name, help="Multiline help text\nFrom the cmd attribute")]
                fn method(&self, args: &[String]) {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(
            parsed.help.unwrap(),
            "Multiline help text\nFrom the cmd attribute".to_string()
        )
    }

    #[test]
    fn should_set_missing_help_text_to_none() {
        let parsed = parse_cmd_attributes(
            &parse_str(
                r###"
                #[cmd(name)]
                fn method(&self, args: &[String]) {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.help, None)
    }

    #[test]
    fn should_parse_alias_from_cmd_attribute() {
        let parsed = parse_cmd_attributes(
            &parse_str(
                r###"
                #[cmd(name, alias("one", "two", three))]
                fn method(&self, args: &[String]) {}
                "###,
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(parsed.alias, vec!["one", "two", "three"]);
    }
}
