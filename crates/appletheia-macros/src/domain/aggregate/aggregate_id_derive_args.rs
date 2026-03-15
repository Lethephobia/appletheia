use syn::spanned::Spanned;
use syn::{Attribute, Path, Result, Type};

#[derive(Debug)]
pub(crate) struct AggregateIdDeriveArgs {
    pub(crate) error: Type,
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

        let error = error.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing `#[aggregate_id(error = ...)]` (or `#[aggregate_id_derive(error = ...)]` when using `#[derive(AggregateId)]` directly)",
            )
        })?;

        Ok(Self { error, validate })
    }
}
