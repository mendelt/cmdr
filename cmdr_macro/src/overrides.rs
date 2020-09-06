use proc_macro2::TokenStream;
use quote::quote;
use syn::{ImplItem, ImplItemMethod, ItemImpl, TypePath};
use crate::parsing::compare_signatures;

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
                    check_signature(&method, "fn prompt(&self) -> String {}");

                    quote!(
                        fn prompt(&self) -> String {
                            #self_type::prompt(&self)
                        }
                    )
                }
                "help" => {
                    check_signature(&method, "fn help(&self, args: &[String]) -> CommandResult {}");

                    quote!(
                        fn help(&self, args: &[String]) -> CommandResult {
                            #self_type::help(&self, args)
                        }
                    )
                }
                "handle_error" => {
                    check_signature(&method, "fn handle_error(&mut self, error: Error) -> CommandResult {}");

                    quote!(
                        fn handle_error(&mut self, error: Error) -> CommandResult {
                            #self_type::handle_error(self, error)
                        }
                    )
                },
                "default" => {
                    check_signature(&method, "fn default(&mut self, command: &Line) -> CommandResult {}");

                    quote!(
                        fn default(&mut self, command: &Line) -> CommandResult {
                            #self_type::default(self, command)
                        }
                    )
                },
                "before_loop" => {
                    check_signature(&method, "fn before_loop(&mut self) {}");

                    quote!(
                        fn before_loop(&mut self) {
                            #self_type::before_loop(self)
                        }
                    )
                }
                "before_command" => {
                    check_signature(&method, "fn before_command(&mut self, line: Line) -> Line {}");

                    quote!(
                        fn before_command(&mut self, line: Line) -> Line {
                            #self_type::before_command(self, line)
                        }
                    )
                }
                "after_command" => {
                    check_signature(&method, "fn after_command(&mut self, line: &Line, result: CommandResult) -> CommandResult {}");

                    quote!(
                        fn after_command(&mut self, line: &Line, result: CommandResult) -> CommandResult {
                            #self_type::after_command(self, line, result)
                        }
                    )
                }
                "after_loop" => { 
                    check_signature(&method, "fn after_loop(&mut self) {}");

                    quote!(
                        fn after_loop(&mut self) {
                            #self_type::after_loop(self)
                        }
                    )
                }
                _ => quote!()
            });
        }
    }

    overrides
}

/// Check the signature of a method against an example string
fn check_signature(method: &ImplItemMethod, expected: &str) {
    let expected_sig: ImplItemMethod = syn::parse_str(expected).unwrap();
    if !compare_signatures(&method.sig, &expected_sig.sig) {
        panic!(
            "Unable to override method \"{}\". Invalid method signature, expected: {}",
            method.sig.ident, expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    /// Normalized test to see if generated tokens are equal to expected code
    fn tokens_eq(generated_tokens: TokenStream, expected: &str) {
        assert_eq!(
            generated_tokens.to_string(),
            syn::parse_str::<TokenStream>(expected).unwrap().to_string()
        );
    }

    #[test]
    fn should_override_prompt_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn prompt(&self) -> String { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(
            format_overrides(&source, &self_type),
            "fn prompt(&self) -> String { SomeImpl::prompt(&self) }",
        );
    }

    #[test]
    #[should_panic]
    fn should_panic_when_overriding_prompt_with_wrong_signature() {
        let source = syn::parse_str("impl SomeImpl {fn prompt(&self) -> bool { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        format_overrides(&source, &self_type).to_string();
    }

    #[test]
    fn should_override_help_when_available() {
        let source =
            syn::parse_str("impl SomeImpl {fn help(&self, args: &[String]) -> CommandResult { }}")
                .unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(
            format_overrides(&source, &self_type),
            "fn help(&self, args: &[String]) -> CommandResult { SomeImpl::help(&self, args) }",
        );
    }

    #[test]
    fn should_override_handle_error_when_available() {
        let source = syn::parse_str(
            "impl SomeImpl {fn handle_error(&mut self, error: Error) -> CommandResult { }}",
        )
        .unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(
            format_overrides(&source, &self_type),
            "fn handle_error(&mut self, error: Error) -> CommandResult { SomeImpl::handle_error(self, error) }"
        );
    }

    #[test]
    fn should_override_default_when_available() {
        let source = syn::parse_str(
            "impl SomeImpl {fn default(&mut self, command: &Line) -> CommandResult { }}",
        )
        .unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(
            format_overrides(&source, &self_type),
            "fn default(&mut self, command: &Line) -> CommandResult { SomeImpl::default(self, command) }"
        );
    }

    #[test]
    fn should_override_before_loop_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn before_loop(&mut self) { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(
            format_overrides(&source, &self_type),
            "fn before_loop(&mut self) { SomeImpl::before_loop(self) }",
        );
    }

    #[test]
    fn should_override_before_command_when_available() {
        let source =
            syn::parse_str("impl SomeImpl {fn before_command(&mut self, line: Line) -> Line { }}")
                .unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(
            format_overrides(&source, &self_type),
            "fn before_command(&mut self, line: Line) -> Line { SomeImpl::before_command(self, line) }"
        );
    }

    #[test]
    fn should_override_after_command_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn after_command(&mut self, line: &Line, result: CommandResult) -> CommandResult { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(
            format_overrides(&source, &self_type),
            "fn after_command(&mut self, line: &Line, result: CommandResult) -> CommandResult { SomeImpl::after_command(self, line, result) }"
        );
    }

    #[test]
    fn should_override_after_loop_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn after_loop(&mut self) { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(
            format_overrides(&source, &self_type),
            "fn after_loop(&mut self) { SomeImpl::after_loop(self) }",
        );
    }

    #[test]
    fn should_override_multiple_commands_when_available() {
        let source = syn::parse_str(
            "impl SomeImpl {fn prompt(&self) -> String { } fn after_loop(&mut self) { }}",
        )
        .unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(
            format_overrides(&source, &self_type),
            r#"
                fn prompt(&self) -> String { SomeImpl::prompt(&self) } 
                fn after_loop(&mut self) { SomeImpl::after_loop(self) }"#,
        );
    }

    #[test]
    fn should_override_nothing_when_no_overridable_methods() {
        let source = syn::parse_str("impl SomeImpl {fn some_other_method() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        tokens_eq(format_overrides(&source, &self_type), "");
    }
}
