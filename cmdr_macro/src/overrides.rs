use proc_macro2::TokenStream;
use quote::quote;
use syn::{ImplItem, ImplItemMethod, ItemImpl, Signature, TypePath, FnArg, PatType, Pat, PatIdent, Ident};
use syn::punctuated::{Pair, Punctuated};

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

fn check_signature(method: &ImplItemMethod, expected: &str) {
    let expected_sig: ImplItemMethod = syn::parse_str(expected).unwrap();

    if !compare_signatures(&method.sig, &expected_sig.sig) {
        panic!(
            "Unable to override method \"{}\". Invalid method signature, expected: {}",
            method.sig.ident, expected
        );
    }
}

/// Compare signatures to see if they're compatible, not equal
fn compare_signatures(signature: &Signature, expected: &Signature) -> bool {

    fn normalize_signature(sig: Signature) -> Signature {
        Signature {
            inputs: sig.inputs.into_pairs().map(normalize_pair).collect(),
            .. sig
        }
    }

    fn normalize_pair<T>(pair: Pair<FnArg, T>) -> Pair<FnArg, T> {
        match pair {
            Pair::Punctuated(arg, token) => Pair::Punctuated(normalize_argument(arg), token),
            Pair::End(arg) => Pair::End(normalize_argument(arg))
        }
    }

    fn normalize_argument(arg: FnArg) -> FnArg {
        match arg {
            FnArg::Receiver(_) => arg,
            FnArg::Typed(pat_type) => FnArg::Typed(
                PatType{
                    pat: normalize_ident(pat_type.pat),
                    .. pat_type
                }),
        }
    }

    fn normalize_ident(pat: Box<Pat>) -> Box<Pat> {
        match pat.as_ref() {
            Pat::Ident(ident) => Box::new(Pat::Ident(
                PatIdent{
                    ident: Ident::new("_", ident.ident.span()), 
                    .. ident.clone()
                })),
            _ => pat
        }
    }

    return normalize_signature(signature.clone()) == normalize_signature(expected.clone());
}

#[cfg(test)]
mod when_comparing_signatures {
    use super::*;

    fn compare_signatures_of(first: &str, second: &str) -> bool {
        compare_signatures(
            &syn::parse_str::<ImplItemMethod>(first).unwrap().sig,
            &syn::parse_str::<ImplItemMethod>(second).unwrap().sig,
        )
    }

    #[test]
    fn should_succeed_for_same_function() {
        assert!(compare_signatures_of(
            "fn func(&mut self, param: i64) -> bool {}",
            "fn func(&mut self, param: i64) -> bool {}"
        ));
    }

    #[test]
    fn should_fail_for_different_functions() {
        assert!(!compare_signatures_of(
            "fn func(&mut self, param: i64) -> bool {}",
            "fn func(&mut self, param: bool) -> i64 {}"
        ));
    }

    #[test]
    fn should_succeed_for_different_parameter_names() {
        assert!(compare_signatures_of(
            "fn func(&mut self, param1: i64, param2: String) -> bool {}",
            "fn func(&mut self, pArAm1: i64, parAM2: String) -> bool {}"
        ));
    }
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
    #[should_panic]
    fn should_panic_when_overriding_prompt_wrong_signature() {
        let source = syn::parse_str("impl SomeImpl {fn prompt(&self) -> bool { }}").unwrap();
        let self_type = util::parse_self_type(&source).unwrap();

        format_overrides(&source, &self_type).to_string();
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
