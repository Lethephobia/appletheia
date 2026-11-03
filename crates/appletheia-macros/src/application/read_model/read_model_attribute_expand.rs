use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Ident, Meta, Path, Result, Token, parse_quote};

use crate::utils::crate_path::resolve_macros_root;

pub(crate) fn expand_read_model_attribute(
    attr: TokenStream,
    item: TokenStream,
    derive_name: &str,
    is_key: bool,
) -> Result<TokenStream> {
    if !attr.is_empty() {
        return Err(syn::Error::new_spanned(
            attr,
            "this attribute does not accept arguments",
        ));
    }

    let mut input: DeriveInput = syn::parse2(item)?;
    if !matches!(input.data, syn::Data::Enum(_)) {
        return Err(syn::Error::new_spanned(
            &input,
            "this attribute requires an enum",
        ));
    }

    let mut existing = HashSet::new();
    for attribute in &input.attrs {
        if attribute.path().is_ident("derive")
            && let Meta::List(list) = &attribute.meta
        {
            let paths = list.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated)?;
            existing.extend(paths.iter().filter_map(|path| {
                path.segments
                    .last()
                    .map(|segment| segment.ident.to_string())
            }));
        }
    }

    let macros_root = resolve_macros_root()?;
    let derive_ident = Ident::new(derive_name, proc_macro2::Span::call_site());
    let mut required: Vec<Path> = vec![parse_quote!(Clone), parse_quote!(Debug)];
    if is_key {
        required.extend([
            parse_quote!(Copy),
            parse_quote!(Eq),
            parse_quote!(PartialEq),
        ]);
    }
    required.push(parse_quote!(#macros_root::#derive_ident));
    let missing: Vec<Path> = required
        .into_iter()
        .filter(|path| {
            path.segments
                .last()
                .is_none_or(|segment| !existing.contains(&segment.ident.to_string()))
        })
        .collect();
    if !missing.is_empty() {
        input.attrs.push(parse_quote!(#[derive(#(#missing),*)]));
    }

    Ok(quote!(#input))
}
