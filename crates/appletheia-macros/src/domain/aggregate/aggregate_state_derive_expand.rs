use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

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

    let error_ty = args.error;

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics #domain::AggregateState for #name #ty_generics #where_clause {
            type Error = #error_ty;
        }
    };

    Ok(expanded)
}
