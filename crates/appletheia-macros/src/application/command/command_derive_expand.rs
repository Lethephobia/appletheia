use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Result};

use super::command_derive_args::CommandDeriveArgs;
use crate::utils::crate_path::resolve_application_path;

pub(crate) fn expand_command_derive(
    input: DeriveInput,
    args: CommandDeriveArgs,
) -> Result<TokenStream> {
    let application = resolve_application_path()?;
    let input_span = input.span();

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Struct(_) = input.data else {
        return Err(syn::Error::new(
            input_span,
            "`Command` can only be derived for structs",
        ));
    };

    let command_name = args.name;

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #application::command::Command for #name #ty_generics #where_clause {
            const NAME: #application::command::CommandName =
                #application::command::CommandName::new(#command_name);
        }
    })
}
