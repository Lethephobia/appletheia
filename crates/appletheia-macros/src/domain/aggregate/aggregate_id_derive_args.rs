use syn::spanned::Spanned;
use syn::{Attribute, Path, Result, Type};

#[derive(Debug)]
pub(crate) struct AggregateIdDeriveArgs {
    pub(crate) error: Option<Type>,
    pub(crate) validate: Option<Path>,
}

impl AggregateIdDeriveArgs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut error: Option<Type> = None;
        let mut validate: Option<Path> = None;

        for attr in attrs {
            if !attr.path().is_ident("aggregate_id_derive") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("error") {
                    let value: Type = meta.value()?.parse()?;
                    if error.is_some() {
                        return Err(syn::Error::new(meta.path.span(), "duplicate `error`"));
                    }
                    error = Some(value);
                    return Ok(());
                }

                if meta.path.is_ident("validate") || meta.path.is_ident("validator") {
                    let value: Path = meta.value()?.parse()?;
                    if validate.is_some() {
                        return Err(syn::Error::new(meta.path.span(), "duplicate `validate`"));
                    }
                    validate = Some(value);
                    return Ok(());
                }

                Err(syn::Error::new(
                    meta.path.span(),
                    "unsupported key (expected `error` or `validate`/`validator`)",
                ))
            })?;
        }

        if validate.is_some() && error.is_none() {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing `#[aggregate_id(error = ...)]` when `validate` is specified (or `#[aggregate_id_derive(error = ...)]` when using `#[derive(AggregateId)]` directly)",
            ));
        }

        Ok(Self { error, validate })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::AggregateIdDeriveArgs;

    #[test]
    fn from_attrs_defaults_error_when_validate_is_absent() {
        let attrs = vec![parse_quote!(#[aggregate_id_derive()])];

        let args = AggregateIdDeriveArgs::from_attrs(&attrs).expect("args should parse");

        assert!(args.error.is_none());
        assert!(args.validate.is_none());
    }

    #[test]
    fn from_attrs_requires_error_when_validate_is_present() {
        let attrs = vec![parse_quote!(#[aggregate_id_derive(validate = validate_counter_id)])];

        let error = AggregateIdDeriveArgs::from_attrs(&attrs).expect_err("args should fail");

        assert!(
            error
                .to_string()
                .contains("missing `#[aggregate_id(error = ...)]` when `validate` is specified")
        );
    }
}
