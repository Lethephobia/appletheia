use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Path, Result, Token, parenthesized};

#[derive(Debug)]
pub(crate) struct UniqueConstraintsAttributeArgs {
    pub(crate) entries: Vec<UniqueConstraintEntryArg>,
}

impl Parse for UniqueConstraintsAttributeArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut entries = Vec::new();

        while !input.is_empty() {
            entries.push(input.parse()?);

            if input.is_empty() {
                break;
            }

            let _ = input.parse::<Token![,]>()?;
        }

        Ok(Self { entries })
    }
}

#[derive(Debug)]
pub(crate) struct UniqueConstraintEntryArg {
    pub(crate) key: LitStr,
    pub(crate) source: UniqueConstraintEntrySourceArg,
}

#[derive(Debug)]
pub(crate) enum UniqueConstraintEntrySourceArg {
    Values(Path),
    Value(Path),
}

impl Parse for UniqueConstraintEntryArg {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let entry: Ident = input.parse()?;
        if entry != "entry" {
            return Err(syn::Error::new(entry.span(), "expected `entry`"));
        }

        let content;
        parenthesized!(content in input);

        let key_ident: Ident = content.parse()?;
        if key_ident != "key" {
            return Err(syn::Error::new(key_ident.span(), "expected `key`"));
        }
        let _ = content.parse::<Token![=]>()?;
        let key = content.parse::<LitStr>()?;

        let _ = content.parse::<Token![,]>()?;

        let source_ident: Ident = content.parse()?;
        let _ = content.parse::<Token![=]>()?;
        let source_path = content.parse::<Path>()?;
        let source = if source_ident == "values" {
            UniqueConstraintEntrySourceArg::Values(source_path)
        } else if source_ident == "value" {
            UniqueConstraintEntrySourceArg::Value(source_path)
        } else {
            return Err(syn::Error::new(
                source_ident.span(),
                "expected `values` or `value`",
            ));
        };

        if !content.is_empty() {
            let _ = content.parse::<Token![,]>()?;
        }

        if !content.is_empty() {
            return Err(content.error("unexpected tokens in `entry(...)`"));
        }

        Ok(Self { key, source })
    }
}
