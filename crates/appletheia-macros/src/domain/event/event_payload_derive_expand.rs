use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Result};

use super::event_payload_derive_args::EventPayloadDeriveArgs;
use crate::utils::crate_path::resolve_domain_path;

pub(crate) fn expand_event_payload_derive(
    input: DeriveInput,
    args: EventPayloadDeriveArgs,
) -> Result<TokenStream> {
    let domain = resolve_domain_path()?;
    let input_span = input.span();

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new(
            input_span,
            "`EventPayload` can only be derived for enums",
        ));
    };

    let const_defs = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let const_ident = format_ident!("{}", to_shouty_snake_case(&variant_ident.to_string()));
        let event_name = to_snake_case(&variant_ident.to_string());

        quote! {
            pub const #const_ident: #domain::EventName = #domain::EventName::new(#event_name);
        }
    });

    let match_arms = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let const_ident = format_ident!("{}", to_shouty_snake_case(&variant_ident.to_string()));
        let pattern = match &variant.fields {
            Fields::Named(_) => quote!(Self::#variant_ident { .. }),
            Fields::Unnamed(_) => quote!(Self::#variant_ident(..)),
            Fields::Unit => quote!(Self::#variant_ident),
        };

        quote! {
            #pattern => Self::#const_ident,
        }
    });

    let error_ty = args.error;

    Ok(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#const_defs)*
        }

        #[automatically_derived]
        impl #impl_generics #domain::EventPayload for #name #ty_generics #where_clause {
            type Error = #error_ty;

            fn name(&self) -> #domain::EventName {
                match self {
                    #(#match_arms)*
                }
            }
        }
    })
}

fn to_snake_case(value: &str) -> String {
    let mut out = String::new();

    for (index, ch) in value.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if index > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }

    out
}

fn to_shouty_snake_case(value: &str) -> String {
    to_snake_case(value).to_ascii_uppercase()
}
