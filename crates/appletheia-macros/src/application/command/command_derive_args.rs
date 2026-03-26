use syn::spanned::Spanned;
use syn::{Attribute, LitStr, Result};

#[derive(Debug)]
pub(crate) struct CommandDeriveArgs {
    pub(crate) name: LitStr,
}

impl CommandDeriveArgs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut name: Option<LitStr> = None;

        for attr in attrs {
            if !attr.path().is_ident("command_derive") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value: LitStr = meta.value()?.parse()?;
                    if name.is_some() {
                        return Err(syn::Error::new(meta.path.span(), "duplicate `name`"));
                    }
                    name = Some(value);
                    return Ok(());
                }

                Err(syn::Error::new(
                    meta.path.span(),
                    "unsupported key (expected `name`)",
                ))
            })?;
        }

        let name = name.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing `#[command(name = \"...\")]` (or `#[command_derive(name = \"...\")]` when using `#[derive(Command)]` directly)",
            )
        })?;

        Ok(Self { name })
    }
}
