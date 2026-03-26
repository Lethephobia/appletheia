use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Path, Result, Token, bracketed};

#[derive(Debug)]
pub(crate) struct RelationsDeriveArgs {
    pub(crate) aggregate: Path,
    pub(crate) relations: Vec<Path>,
}

impl RelationsDeriveArgs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut aggregate: Option<Path> = None;
        let mut relations: Option<Vec<Path>> = None;

        for attr in attrs {
            if !attr.path().is_ident("relations_derive") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("aggregate") {
                    let value: Path = meta.value()?.parse()?;
                    if aggregate.is_some() {
                        return Err(syn::Error::new(meta.path.span(), "duplicate `aggregate`"));
                    }
                    aggregate = Some(value);
                    return Ok(());
                }

                if meta.path.is_ident("relations") {
                    let value: RelationPathList = meta.value()?.parse()?;
                    if relations.is_some() {
                        return Err(syn::Error::new(meta.path.span(), "duplicate `relations`"));
                    }
                    relations = Some(value.0);
                    return Ok(());
                }

                Err(syn::Error::new(
                    meta.path.span(),
                    "unsupported key (expected `aggregate` or `relations`)",
                ))
            })?;
        }

        let aggregate = aggregate.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing `#[relations(aggregate = ...)]` (or `#[relations_derive(aggregate = ...)]` when using `#[derive(Relations)]` directly)",
            )
        })?;

        let relations = relations.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing `#[relations(relations = [...])]` (or `#[relations_derive(relations = [...])]` when using `#[derive(Relations)]` directly)",
            )
        })?;

        Ok(Self {
            aggregate,
            relations,
        })
    }
}

#[derive(Debug)]
struct RelationPathList(Vec<Path>);

impl Parse for RelationPathList {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        bracketed!(content in input);
        let relations = Punctuated::<Path, Token![,]>::parse_terminated(&content)?;
        Ok(Self(relations.into_iter().collect()))
    }
}
