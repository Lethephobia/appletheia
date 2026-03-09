use syn::spanned::Spanned;
use syn::{Attribute, Result, Type};

#[derive(Debug)]
pub(crate) struct EventPayloadDeriveArgs {
    pub(crate) error: Type,
}

impl EventPayloadDeriveArgs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut error: Option<Type> = None;

        for attr in attrs {
            if !attr.path().is_ident("event_payload_derive") {
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

                Err(syn::Error::new(
                    meta.path.span(),
                    "unsupported key (expected `error`)",
                ))
            })?;
        }

        let error = error.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing `#[event_payload(error = ...)]` (or `#[event_payload_derive(error = ...)]` when using `#[derive(EventPayload)]` directly)",
            )
        })?;

        Ok(Self { error })
    }
}
