use quote::quote;
use syn::export::TokenStream2;
use syn::{ImplItem, ItemImpl, TypePath};

pub fn format_overrides(input: &ItemImpl, self_type: &TypePath) -> TokenStream2 {
    let mut overrides = TokenStream2::new();

    overrides.extend(format_prompt_override(&input, self_type));
    overrides.extend(format_empty_override(&input, self_type));
    overrides.extend(format_default_override(&input, self_type));

    overrides.extend(format_before_loop_override(&input, self_type));
    overrides.extend(format_before_command_override(&input, self_type));
    overrides.extend(format_after_command_override(&input, self_type));
    overrides.extend(format_after_loop_override(&input, self_type));

    overrides
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
