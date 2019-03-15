use quote::quote;
use syn::export::TokenStream2;
use syn::{ImplItem, ItemImpl, Type, TypePath};

pub fn format_overrides(input: &ItemImpl) -> TokenStream2 {
    let mut overrides = TokenStream2::new();

    if let Type::Path(self_type) = &*input.self_ty {
        overrides.extend(format_prompt_override(&input, self_type));
        overrides.extend(format_empty_override(&input, self_type));
        overrides.extend(format_default_override(&input, self_type));

        overrides.extend(format_before_loop_override(&input, self_type));
        overrides.extend(format_before_command_override(&input, self_type));
        overrides.extend(format_after_command_override(&input, self_type));
        overrides.extend(format_after_loop_override(&input, self_type));
    }

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
                #self_type::before_loop(self)
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

    #[test]
    fn should_override_prompt_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn prompt() { }}").unwrap();

        assert_eq!(
            format_overrides(&source).to_string(),
            "fn prompt ( & self ) -> String { SomeImpl :: prompt ( & self ) }"
        );
    }

    #[test]
    fn should_override_empty_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn empty() { }}").unwrap();

        assert_eq!(
            format_overrides(&source).to_string(),
            "fn empty ( & mut self ) -> CommandResult { SomeImpl :: empty ( & self ) }"
        );
    }

    #[test]
    fn should_override_default_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn default() { }}").unwrap();

        assert_eq!(
            format_overrides(&source).to_string(),
            "fn default ( & mut self , command : & CommandLine ) -> CommandResult { SomeImpl :: default ( self , command ) }"
        );
    }

    #[test]
    fn should_override_before_loop_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn before_loop() { }}").unwrap();

        assert_eq!(
            format_overrides(&source).to_string(),
            "fn before_loop ( & mut self ) { SomeImpl :: before_loop ( self ) }"
        );
    }

    #[test]
    fn should_override_before_command_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn before_command() { }}").unwrap();

        assert_eq!(
            format_overrides(&source).to_string(),
            "fn before_command ( & mut self , line : Line ) -> Line { SomeImpl :: before_command ( self , line ) }"
        );
    }

    #[test]
    fn should_override_after_command_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn after_command() { }}").unwrap();

        assert_eq!(
            format_overrides(&source).to_string(),
            "fn after_command ( & mut self , line : & Line , result : CommandResult ) -> CommandResult { SomeImpl :: after_command ( self , line , result ) }"
        );
    }

    #[test]
    fn should_override_after_loop_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn after_loop() { }}").unwrap();

        assert_eq!(
            format_overrides(&source).to_string(),
            "fn after_loop ( & mut self ) { SomeImpl :: after_loop ( self ) }"
        );
    }

    #[test]
    fn should_override_multiple_commands_when_available() {
        let source =
            syn::parse_str("impl SomeImpl {fn after_loop() { } fn prompt() { } }").unwrap();

        assert_eq!(
            format_overrides(&source).to_string(),
            "fn prompt ( & self ) -> String { SomeImpl :: prompt ( & self ) } fn after_loop ( & mut self ) { SomeImpl :: after_loop ( self ) }"
        );
    }

    #[test]
    fn should_override_nothing_when_no_overridable_methods() {
        let source = syn::parse_str("impl SomeImpl {fn some_other_method() { }}").unwrap();

        assert_eq!(format_overrides(&source).to_string(), "");
    }
}
