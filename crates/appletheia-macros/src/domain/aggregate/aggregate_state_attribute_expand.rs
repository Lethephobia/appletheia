use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Item, Meta, Path, Result, Token};

use crate::utils::crate_path::{resolve_macros_root, resolve_serde_root};

pub(crate) fn expand_aggregate_state_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    let attr_tokens: TokenStream = attr.into();

    let mut item: Item = syn::parse(item)?;
    let Item::Struct(ref mut item) = item else {
        return Err(syn::Error::new(
            item.span(),
            "`#[aggregate_state(...)]` can only be applied to a struct",
        ));
    };

    let already_has_helper = item
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("aggregate_state_derive"));
    if already_has_helper {
        return Err(syn::Error::new(
            item.span(),
            "`#[aggregate_state_derive(...)]` must not be used together with `#[aggregate_state(...)]`",
        ));
    }

    let existing_derive_keys = collect_derive_keys(&item.attrs)?;
    let macros_root = resolve_macros_root()?;
    let serde_root = resolve_serde_root()?;

    let required: Vec<Path> = vec![
        syn::parse_quote!(Clone),
        syn::parse_quote!(Debug),
        syn::parse_quote!(Eq),
        syn::parse_quote!(PartialEq),
        syn::parse_quote!(Hash),
        syn::parse_quote!(#serde_root::Serialize),
        syn::parse_quote!(#serde_root::Deserialize),
        syn::parse_quote!(#macros_root::AggregateState),
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
        item.attrs.push(syn::parse_quote!(#[derive(#(#missing),*)]));
    }

    item.attrs
        .push(syn::parse_quote!(#[aggregate_state_derive(#attr_tokens)]));

    Ok(quote!(#item))
}

fn collect_derive_keys(attrs: &[Attribute]) -> Result<std::collections::HashSet<String>> {
    let mut keys = std::collections::HashSet::new();

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
