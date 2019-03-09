extern crate proc_macro;
extern crate proc_macro2;

use self::proc_macro::TokenStream;
use self::proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, ImplItem, ItemImpl, Type};
use syn::{ImplItemMethod, Meta, TypePath};

/// Implements the cmdr::Scope trait on any impl block.
///
/// The macro can be used to annotate any plain impl block it will then generate an additional
/// impl block to implement Scope for the same type.
///
/// Right now it will search the impl block for methods starting with do_ and call them in a
/// generated Scope::command method when the right command is received.
#[proc_macro_attribute]
pub fn cmdr(_meta: TokenStream, code: TokenStream) -> TokenStream {
    let input = parse_macro_input!(code as ItemImpl);
    let command_methods = get_methods(&input);

    if let Type::Path(self_type) = &*input.self_ty {
        let prompt_override = format_prompt_override(&input, self_type);
        let empty_override = format_empty_override(&input, self_type);
        let default_override = format_default_override(&input, self_type);

        let before_loop_override = format_before_loop_override(&input, self_type);
        let before_command_override = format_before_command_override(&input, self_type);
        let after_command_override = format_after_command_override(&input, self_type);
        let after_loop_override = format_after_loop_override(&input, self_type);

        TokenStream::from(quote!(
            #input

            impl cmdr::Scope for #self_type {
                fn commands() -> CmdMethodList<#self_type> {
                    CmdMethodList::new(vec![
                        #(#command_methods)*
                    ])
                }

                #prompt_override
                #empty_override
                #default_override

                #before_loop_override
                #before_command_override
                #after_loop_override
                #after_command_override
            }
        ))
    } else {
        panic!("Unable to parse impl type")
    }
}

#[proc_macro_attribute]
pub fn cmd(_meta: TokenStream, code: TokenStream) -> TokenStream { code }

fn format_prompt_override(input: &ItemImpl, self_type: &TypePath) -> TokenStream2 {
    if contains_method(&input, "prompt") {
        quote!(
            fn prompt(&self) -> String {
                #self_type::prompt(&self)
            }
        )
    } else {
        quote!()
    }
}

fn format_empty_override(input: &ItemImpl, self_type: &TypePath) -> TokenStream2 {
    if contains_method(&input, "empty") {
        quote!(
            fn empty(&mut self) -> CommandResult {
                #self_type::empty(&self)
            }
        )
    } else {
        quote!()
    }
}

fn format_default_override(input: &ItemImpl, self_type: &TypePath) -> TokenStream2 {
    if contains_method(&input, "default") {
        quote!(
            fn default(&mut self, command: &CommandLine) -> CommandResult {
                #self_type::default(self, command)
            }
        )
    } else {
        quote!()
    }
}

fn format_before_loop_override(input: &ItemImpl, self_type: &TypePath) -> TokenStream2 {
    if contains_method(&input, "before_loop") {
        quote!(
            fn before_loop(&mut self) {
                #self_type::before_loop(self);
            }
        )
    } else {
        quote!()
    }
}

fn format_before_command_override(input: &ItemImpl, self_type: &TypePath) -> TokenStream2 {
    if contains_method(&input, "before_command") {
        quote!(
            fn before_command(&mut self, line: Line) -> Line {
                #self_type::before_command(self, line)
            }
        )
    } else {
        quote!()
    }
}

fn format_after_command_override(input: &ItemImpl, self_type: &TypePath) -> TokenStream2 {
    if contains_method(&input, "after_command") {
        quote!(
            fn after_command(&mut self, line: &Line, result: CommandResult) -> CommandResult {
                #self_type::after_command(self, line, result)
            }
        )
    } else {
        quote!()
    }
}

fn format_after_loop_override(input: &ItemImpl, self_type: &TypePath) -> TokenStream2 {
    if contains_method(&input, "after_loop") {
        quote!(
            fn after_loop(&mut self) {
                #self_type::after_loop(self)
            }
        )
    } else {
        quote!()
    }
}

fn contains_method(input: &ItemImpl, method_name: &str) -> bool {
    for item in &input.items {
        if let ImplItem::Method(method) = item {
            let ident = &method.sig.ident;
            let name = ident.to_string();

            if name == method_name {
                return true;
            }
        }
    }
    false
}

fn get_methods(input: &ItemImpl) -> Vec<(CmdMeta)> {
    let mut result = Vec::new();

    for item in &input.items {
        if let ImplItem::Method(method) = item {
            dbg!("Stuff");
            if let Some(metadata) = parse_cmd_attribute(method) {
                println!("Stuff {}", metadata.method);
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

    match cmd_attr {
        Some(attr) => Some(CmdMeta {
            command: methodname.to_string(),
            method: methodname.to_owned(),
        }),
        None => None,
    }
}

struct CmdMeta {
    command: String,
    method: Ident,
}

impl ToTokens for CmdMeta {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let command = &self.command;
        let method = &self.method;

        tokens.extend(quote!(CmdMethod::new(#command.to_string(), Box::new(|scope, cmd_line| scope.#method(&cmd_line.args)),),))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_itemimpl() -> ItemImpl {
        syn::parse_str(
            r###"
            impl SomeImpl {
                fn prompt() { }
            }
            "###,
        )
        .unwrap()
    }

    #[test]
    fn test_contains_method_returns_true_when_method_found() {
        assert!(contains_method(&test_itemimpl(), "prompt"));
    }

    #[test]
    fn test_contains_method_returns_false_when_method_not_found() {
        assert!(!contains_method(&test_itemimpl(), "fn_not_there"));
    }
}
