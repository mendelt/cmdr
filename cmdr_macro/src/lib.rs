extern crate proc_macro;
extern crate proc_macro2;

use self::proc_macro::TokenStream;
use self::proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::TypePath;
use syn::{parse_macro_input, Ident, ImplItem, ItemImpl, Type};

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
    let command_matches = format_command_match(&get_methods(&input));

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
                fn command(&mut self, command: &CommandLine) -> CommandResult {
                    match &command.command[..] {
                        #(#command_matches)*
                    }
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

fn format_command_match(methods: &[(Ident, String)]) -> Vec<TokenStream2> {
    let mut result: Vec<TokenStream2> = Vec::new();

    // Add match clauses for all do_methods
    for (method, name) in methods {
        result.push(quote!(#name => self.#method(&command.args),));
    }

    // Add the catch all
    result.push(quote!(_ => self.default(command)));

    result
}

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
            fn before_command(&mut self, line: &Line) {
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
