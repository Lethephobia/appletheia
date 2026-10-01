use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Item, Meta, Path, Result, Token};

use crate::utils::crate_path::{resolve_macros_root, resolve_serde_root};

pub(crate) fn expand_saga_step_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    let attr_tokens: TokenStream = attr.into();
    if !attr_tokens.is_empty() {
        return Err(syn::Error::new_spanned(
            attr_tokens,
            "`#[saga_step]` does not accept arguments",
        ));
    }

    let mut item: Item = syn::parse(item)?;
    let Item::Enum(ref mut item) = item else {
        return Err(syn::Error::new(
            item.span(),
            "`#[saga_step]` can only be applied to an enum",
        ));
    };

    let existing_derive_keys = collect_derive_keys(&item.attrs)?;
    let already_has_serde_attr = item.attrs.iter().any(|attr| attr.path().is_ident("serde"));

    let macros_root = resolve_macros_root()?;
    let serde_root = resolve_serde_root()?;

    let required: Vec<Path> = vec![
        syn::parse_quote!(Copy),
        syn::parse_quote!(Clone),
        syn::parse_quote!(Debug),
        syn::parse_quote!(Eq),
        syn::parse_quote!(PartialEq),
        syn::parse_quote!(#serde_root::Serialize),
        syn::parse_quote!(#serde_root::Deserialize),
        syn::parse_quote!(#macros_root::SagaStep),
    ];

    let missing: Vec<Path> = required
        .into_iter()
        .filter(|path| {
            let Some(key) = derive_key(path) else {
                return true;
            };
            !existing_derive_keys.contains(&key)
        })
        .collect();

    if !missing.is_empty() {
        item.attrs
            .insert(0, syn::parse_quote!(#[derive(#(#missing),*)]));
    }

    if !already_has_serde_attr {
        item.attrs.push(
            syn::parse_quote!(#[serde(tag = "type", content = "data", rename_all = "snake_case")]),
        );
    }

    Ok(quote!(#item))
}

fn collect_derive_keys(attrs: &[Attribute]) -> Result<HashSet<String>> {
    let mut keys = HashSet::new();

    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }

        let Meta::List(list) = &attr.meta else {
            continue;
        };

        let paths: Punctuated<Path, Token![,]> =
            list.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)?;
        keys.extend(paths.iter().filter_map(derive_key));
    }

    Ok(keys)
}

fn derive_key(path: &Path) -> Option<String> {
    path.segments.last().map(|seg| seg.ident.to_string())
}
