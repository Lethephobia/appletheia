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
    pub(crate) values: Path,
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

        let values_ident: Ident = content.parse()?;
        if values_ident != "values" {
            return Err(syn::Error::new(values_ident.span(), "expected `values`"));
        }
        let _ = content.parse::<Token![=]>()?;
        let values = content.parse::<Path>()?;

        if !content.is_empty() {
            let _ = content.parse::<Token![,]>()?;
        }

        if !content.is_empty() {
            return Err(content.error("unexpected tokens in `entry(...)`"));
        }

        Ok(Self { key, values })
    }
}
