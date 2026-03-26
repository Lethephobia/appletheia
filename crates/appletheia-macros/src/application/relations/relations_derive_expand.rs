use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Result};

use super::relations_derive_args::RelationsDeriveArgs;
use crate::utils::crate_path::{resolve_application_path, resolve_domain_path};

pub(crate) fn expand_relations_derive(
    input: DeriveInput,
    args: RelationsDeriveArgs,
) -> Result<TokenStream> {
    let application = resolve_application_path()?;
    let domain = resolve_domain_path()?;
    let input_span = input.span();

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Struct(_) = input.data else {
        return Err(syn::Error::new(
            input_span,
            "`Relations` can only be derived for structs",
        ));
    };

    let aggregate = args.aggregate;
    let relations = args.relations;

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #application::authorization::Relations for #name #ty_generics #where_clause {
            const AGGREGATE_TYPE: #domain::AggregateType =
                <#aggregate as #domain::Aggregate>::TYPE;

            fn build(&self) -> #application::authorization::AuthorizationTypeDefinition {
                let mut definition = #application::authorization::AuthorizationTypeDefinition::default();
                #(definition.define_static_relation(#relations);)*
                definition
            }
        }
    })
}
