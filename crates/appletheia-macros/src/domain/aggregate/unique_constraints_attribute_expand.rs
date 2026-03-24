use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Item, ItemStruct, Result};

use super::unique_constraints_attribute_args::UniqueConstraintsAttributeArgs;
use crate::utils::crate_path::resolve_domain_path;

pub(crate) fn expand_unique_constraints_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    let args: UniqueConstraintsAttributeArgs = syn::parse(attr)?;
    let item: Item = syn::parse(item)?;
    let Item::Struct(item_struct) = item else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "`#[unique_constraints(...)]` can only be applied to a struct",
        ));
    };

    expand_unique_constraints_impl(item_struct, args)
}

fn expand_unique_constraints_impl(
    item_struct: ItemStruct,
    args: UniqueConstraintsAttributeArgs,
) -> Result<TokenStream> {
    let domain = resolve_domain_path()?;

    let name = &item_struct.ident;
    let generics = &item_struct.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let inserts = args.entries.iter().map(|entry| {
        let key = &entry.key;
        let values = &entry.values;

        quote! {
            if let Some(values) = #values(self)? {
                let _ = entries.insert(#domain::UniqueKey::new(#key), values);
            }
        }
    });

    let key_consts = args.entries.iter().map(|entry| {
        let key = entry.key.value();
        let const_ident = format_ident!("{}_KEY", to_shouty_snake_case(&key));
        let key_literal = &entry.key;

        quote! {
            pub const #const_ident: #domain::UniqueKey = #domain::UniqueKey::new(#key_literal);
        }
    });

    Ok(quote! {
        #item_struct

        impl #impl_generics #name #ty_generics #where_clause {
            #(#key_consts)*
        }

        #[automatically_derived]
        impl #impl_generics #domain::UniqueConstraints<<#name #ty_generics as #domain::AggregateState>::Error>
            for #name #ty_generics #where_clause
        {
            fn unique_entries(
                &self,
            ) -> ::std::result::Result<
                #domain::UniqueEntries,
                <#name #ty_generics as #domain::AggregateState>::Error,
            > {
                let mut entries = #domain::UniqueEntries::new();
                #(#inserts)*
                Ok(entries)
            }
        }
    })
}

fn to_shouty_snake_case(value: &str) -> String {
    value.to_ascii_uppercase()
}
