use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Ident, Result, Type};

use super::aggregate_state_derive_args::AggregateStateDeriveArgs;
use crate::utils::crate_path::resolve_domain_path;

pub(crate) fn expand_aggregate_state_derive(
    input: DeriveInput,
    args: AggregateStateDeriveArgs,
) -> Result<TokenStream> {
    let domain = resolve_domain_path()?;

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let id_field = args.id_field;
    let id_ty = extract_id_ty(&input.data, &id_field)?;
    let error_ty = args.error;

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics #domain::AggregateState for #name #ty_generics #where_clause {
            type Id = #id_ty;
            type Error = #error_ty;

            fn id(&self) -> Self::Id {
                self.#id_field
            }
        }
    };

    Ok(expanded)
}

fn extract_id_ty(data: &Data, id_field: &Ident) -> Result<Type> {
    let fields = match data {
        Data::Struct(data) => &data.fields,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "`AggregateState` can only be derived for structs",
            ));
        }
    };

    let named_fields = match fields {
        Fields::Named(fields) => &fields.named,
        _ => {
            return Err(syn::Error::new(
                fields.span(),
                "`AggregateState` requires a struct with named fields",
            ));
        }
    };

    let id = named_fields
        .iter()
        .find(|field| field.ident.as_ref().is_some_and(|ident| ident == id_field));

    let id = id.ok_or_else(|| {
        syn::Error::new(
            named_fields.span(),
            format!("missing `{id_field}` field for `AggregateState`"),
        )
    })?;

    Ok(id.ty.clone())
}
