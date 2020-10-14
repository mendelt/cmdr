//! Helper methods for parsing rust code using syn
use syn::punctuated::Pair;
use syn::{FnArg, Ident, ItemImpl, Pat, PatIdent, PatType, Signature, Type, TypePath};

pub(crate) fn parse_self_type(input: &ItemImpl) -> Option<TypePath> {
    match &*input.self_ty {
        Type::Path(self_type) => Some(self_type.to_owned()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_impl_self_type() {
        let source = &syn::parse_str("impl SomeImpl {}").unwrap();

        assert_eq!(
            parse_self_type(source),
            Some(syn::parse_str("SomeImpl").unwrap())
        );
    }
}

/// Compare signatures to see if they're compatible, not equal
pub(crate) fn compare_signatures(signature: &Signature, expected: &Signature) -> bool {
    fn normalize_signature(sig: Signature) -> Signature {
        Signature {
            inputs: sig.inputs.into_pairs().map(normalize_pair).collect(),
            ..sig
        }
    }

    fn normalize_pair<T>(pair: Pair<FnArg, T>) -> Pair<FnArg, T> {
        match pair {
            Pair::Punctuated(arg, token) => Pair::Punctuated(normalize_argument(arg), token),
            Pair::End(arg) => Pair::End(normalize_argument(arg)),
        }
    }

    fn normalize_argument(arg: FnArg) -> FnArg {
        match arg {
            FnArg::Receiver(_) => arg,
            FnArg::Typed(pat_type) => FnArg::Typed(PatType {
                pat: normalize_ident(pat_type.pat),
                ..pat_type
            }),
        }
    }

    fn normalize_ident(pat: Box<Pat>) -> Box<Pat> {
        match pat.as_ref() {
            Pat::Ident(ident) => Box::new(Pat::Ident(PatIdent {
                ident: Ident::new("_", ident.ident.span()),
                ..ident.clone()
            })),
            _ => pat,
        }
    }

    normalize_signature(signature.clone()) == normalize_signature(expected.clone())
}

#[cfg(test)]
mod when_comparing_signatures {
    use super::*;
    use syn::ImplItemMethod;

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
