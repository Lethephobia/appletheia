use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Item, Meta, Path, Result, Token};

use crate::utils::crate_path::resolve_macros_root;

pub(crate) fn expand_command_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    let attr_tokens: TokenStream = attr.into();

    let mut item: Item = syn::parse(item)?;
    let Item::Struct(ref mut item) = item else {
        return Err(syn::Error::new(
            item.span(),
            "`#[command(...)]` can only be applied to a struct",
        ));
    };

    let already_has_helper = item
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("command_derive"));
    if already_has_helper {
        return Err(syn::Error::new(
            item.span(),
            "`#[command_derive(...)]` must not be used together with `#[command(...)]`",
        ));
    }

    let existing_derive_keys = collect_derive_keys(&item.attrs)?;
    let macros_root = resolve_macros_root()?;
    let command_derive: Path = syn::parse_quote!(#macros_root::Command);

    let should_add_command_derive = derive_key(&command_derive)
        .map(|key| !existing_derive_keys.contains(&key))
        .unwrap_or(true);

    if should_add_command_derive {
        item.attrs
            .push(syn::parse_quote!(#[derive(#command_derive)]));
    }

    item.attrs
        .push(syn::parse_quote!(#[command_derive(#attr_tokens)]));

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
