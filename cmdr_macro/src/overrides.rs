use proc_macro2::TokenStream;
use quote::quote;
use syn::{ImplItem, ItemImpl, TypePath};

pub fn format_overrides(input: &ItemImpl, self_type: &TypePath) -> TokenStream {
    let mut overrides = TokenStream::new();

    for item in &input.items {
        if let ImplItem::Method(method) = item {
            overrides.extend(match method.sig.ident.to_string().as_ref() {
                "prompt" => quote!(
                    fn prompt(&self) -> String {
                        #self_type::prompt(&self)
                    }
                ),
                "help" => quote!(
                    fn help(&self, args: &[String]) -> CommandResult {
                        #self_type::help(&self, args)
                    }
                ),
                "empty" => quote!(
                    fn empty(&mut self) -> CommandResult {
                        #self_type::empty(&self)
                    }
                ),
                "handle_error" => quote!(
                    fn handle_error(&mut self, error: CommandError) -> CommandResult {
                        #self_type::handle_error(self, error)
                    }
                ),
                "default" => quote!(
                    fn default(&mut self, command: &CommandLine) -> CommandResult {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn should_override_prompt_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn prompt() { }}").unwrap();
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
    fn should_override_empty_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn empty() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn empty ( & mut self ) -> CommandResult { SomeImpl :: empty ( & self ) }"
        );
    }

    #[test]
    fn should_override_handle_error_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn handle_error() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn handle_error ( & mut self , error : CommandError ) -> CommandResult { SomeImpl :: handle_error ( self , error ) }"
        );
    }

    #[test]
    fn should_override_default_when_available() {
        let source = syn::parse_str("impl SomeImpl {fn default() { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        assert_eq!(
            format_overrides(&source, &self_type).to_string(),
            "fn default ( & mut self , command : & CommandLine ) -> CommandResult { SomeImpl :: default ( self , command ) }"
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
        let source = syn::parse_str("impl SomeImpl {fn prompt() { } fn after_loop() { }}").unwrap();
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
