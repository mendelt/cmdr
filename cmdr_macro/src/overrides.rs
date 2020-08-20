use proc_macro2::TokenStream;
use quote::quote;
use syn::{ImplItem, ImplItemMethod, ItemImpl, TypePath};

/// Checks the cmdr type to see if any override methods are available. Override methods
/// are methods that override a method that has a default implementation in the Scope trait.
/// When an override is available in the type we're implementing Scope for we generate a method
/// that calls the user supplied functionality.
pub(crate) fn format_overrides(input: &ItemImpl, self_type: &TypePath) -> TokenStream {
    let mut overrides = TokenStream::new();

    for item in &input.items {
        if let ImplItem::Method(method) = item {
            overrides.extend(match method.sig.ident.to_string().as_ref() {
                "prompt" => {
                    check_signature(method, "fn prompt(&self) -> String {}");
                    quote!(
                        fn prompt(&self) -> String {
                            #self_type::prompt(&self)
                        }
                    )
                }
                "help" => quote!(
                    fn help(&self, args: &[String]) -> CommandResult {
                        #self_type::help(&self, args)
                    }
                ),
                "handle_error" => quote!(
                    fn handle_error(&mut self, error: Error) -> CommandResult {
                        #self_type::handle_error(self, error)
                    }
                ),
                "default" => quote!(
                    fn default(&mut self, command: &Line) -> CommandResult {
                        #self_type::default(self, command)
                    }
                ),
                "before_loop" => quote!(
                    fn before_loop(&mut self) {
                        #self_type::before_loop(self)
                    }
                ),
                "before_command" => quote!(
                    fn before_command(&mut self, line: Line) -> Line {
                        #self_type::before_command(self, line)
                    }
                ),
                "after_command" => quote!(
                    fn after_command(&mut self, line: &Line, result: CommandResult) -> CommandResult {
                       #self_type::after_command(self, line, result)
                    }
                ),
                "after_loop" => quote!(
                    fn after_loop(&mut self) {
                       #self_type::after_loop(self)
                    }
                ),
                _ => quote!()
            });
        }
    }

    overrides
}

fn check_signature(method: &ImplItemMethod, signature: &str) {
    let expected: ImplItemMethod = syn::parse_str(signature).unwrap();

    assert_eq!(method.sig, expected.sig);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn should_override_prompt_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn prompt(&self) -> String { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn prompt ( & self ) -> String { SomeImpl :: prompt ( & self ) }"
        );
    }

    #[test]
    fn should_override_help_when_available() {
        let source = syn::parse_str(
            "impl SomeImpl {fn help(args: &[String]) -> CommandResult { CommandResult::Ok }}",
        )
        .unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn help ( & self , args : & [ String ] ) -> CommandResult { SomeImpl :: help ( & self , args ) }"
        );
    }

    #[test]
    fn should_override_handle_error_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn handle_error() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn handle_error ( & mut self , error : Error ) -> CommandResult { SomeImpl :: handle_error ( self , error ) }"
        );
    }

    #[test]
    fn should_override_default_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn default() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn default ( & mut self , command : & Line ) -> CommandResult { SomeImpl :: default ( self , command ) }"
        );
    }

    #[test]
    fn should_override_before_loop_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn before_loop() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn before_loop ( & mut self ) { SomeImpl :: before_loop ( self ) }"
        );
    }

    #[test]
    fn should_override_before_command_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn before_command() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn before_command ( & mut self , line : Line ) -> Line { SomeImpl :: before_command ( self , line ) }"
        );
    }

    #[test]
    fn should_override_after_command_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn after_command() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn after_command ( & mut self , line : & Line , result : CommandResult ) -> CommandResult { SomeImpl :: after_command ( self , line , result ) }"
        );
    }

    #[test]
    fn should_override_after_loop_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn after_loop() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn after_loop ( & mut self ) { SomeImpl :: after_loop ( self ) }"
        );
    }

    #[test]
    fn should_override_multiple_commands_when_available() {
        let source =
            syn::parse_str("impl SomeImpl {fn prompt(&self) -> String { } fn after_loop() { }}")
                .unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn prompt ( & self ) -> String { SomeImpl :: prompt ( & self ) } fn after_loop ( & mut self ) { SomeImpl :: after_loop ( self ) }"
        );
    }

    #[test]
    fn should_override_nothing_when_no_overridable_methods() {
        let source = syn::parse_str("impl SomeImpl {fn some_other_method() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(format_overrides(&source, &self_type).to_string(), "");
    }
}
