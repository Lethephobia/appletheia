use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Item, ItemStruct, Result};

use super::reference_indexes_attribute_args::{
    ReferenceIndexEntrySourceArg, ReferenceIndexesAttributeArgs,
};
use crate::utils::crate_path::resolve_domain_path;

pub(crate) fn expand_reference_indexes_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    let args: ReferenceIndexesAttributeArgs = syn::parse(attr)?;
    let item: Item = syn::parse(item)?;
    let Item::Struct(item_struct) = item else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "`#[reference_indexes(...)]` can only be applied to a struct",
        ));
    };

    expand_reference_indexes_impl(item_struct, args)
}

fn expand_reference_indexes_impl(
    item_struct: ItemStruct,
    args: ReferenceIndexesAttributeArgs,
) -> Result<TokenStream> {
    let domain = resolve_domain_path()?;

    let name = &item_struct.ident;
    let generics = &item_struct.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let inserts = args.entries.iter().map(|entry| {
        let key = &entry.key;
        match &entry.source {
            ReferenceIndexEntrySourceArg::Values(values) => quote! {
                if let Some(values) = #values(self)? {
                    let _ = entries.insert(#domain::ReferenceKey::new(#key), values);
                }
            },
            ReferenceIndexEntrySourceArg::Value(value) => quote! {
                if let Some(value) = #value(self)? {
                    let values = #domain::ReferenceValues::new(vec![value])?;
                    let _ = entries.insert(#domain::ReferenceKey::new(#key), values);
                }
            },
        }
    });

    let ref_consts = args.entries.iter().map(|entry| {
        let key = entry.key.value();
        let const_ident = format_ident!("{}_REF", to_shouty_snake_case(&key));
        let key_literal = &entry.key;

        quote! {
            pub const #const_ident: #domain::ReferenceKey = #domain::ReferenceKey::new(#key_literal);
        }
    });

    Ok(quote! {
        #item_struct

        impl #impl_generics #name #ty_generics #where_clause {
            #(#ref_consts)*
        }

        #[automatically_derived]
        impl #impl_generics #domain::ReferenceIndexes<<#name #ty_generics as #domain::AggregateState>::Error>
            for #name #ty_generics #where_clause
        {
            fn reference_entries(
                &self,
            ) -> ::std::result::Result<
                #domain::ReferenceEntries,
                <#name #ty_generics as #domain::AggregateState>::Error,
            > {
                let mut entries = #domain::ReferenceEntries::new();
                #(#inserts)*
                Ok(entries)
            }
        }
    })
}

fn to_shouty_snake_case(value: &str) -> String {
    value.to_ascii_uppercase()
}
