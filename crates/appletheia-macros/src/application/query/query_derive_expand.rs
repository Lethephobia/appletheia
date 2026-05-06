use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Result};

use super::query_derive_args::QueryDeriveArgs;
use crate::utils::crate_path::resolve_application_path;

pub(crate) fn expand_query_derive(
    input: DeriveInput,
    args: QueryDeriveArgs,
) -> Result<TokenStream> {
    let application = resolve_application_path()?;
    let input_span = input.span();

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Struct(_) = input.data else {
        return Err(syn::Error::new(
            input_span,
            "`Query` can only be derived for structs",
        ));
    };

    let query_name = args.name;

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #application::query::Query for #name #ty_generics #where_clause {
            const NAME: #application::query::QueryName =
                #application::query::QueryName::new(#query_name);
        }
    })
}
