use syn::{ItemImpl, TypePath, Type};

pub fn parse_self_type(input: &ItemImpl) -> Option<TypePath> {
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
