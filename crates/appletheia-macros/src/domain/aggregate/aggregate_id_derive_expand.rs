use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Result, Type, TypePath};

use crate::domain::aggregate::aggregate_id_derive_args::AggregateIdDeriveArgs;
use crate::utils::crate_path::{resolve_domain_path, resolve_uuid_root};

pub(crate) fn expand_aggregate_id_derive(
    input: DeriveInput,
    args: AggregateIdDeriveArgs,
) -> Result<TokenStream> {
    let domain = resolve_domain_path()?;
    let uuid = resolve_uuid_root()?;

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let inner_ty = extract_inner_uuid_ty(&input.data)?;

    let has_custom_error = args.error.is_some();
    let error_ty: Type = args
        .error
        .unwrap_or_else(|| syn::parse_quote!(::std::convert::Infallible));
    let validate = args.validate;
    let validate_stmt = validate.map(|path| quote!(#path(value)?;));
    let try_from_allow = if has_custom_error {
        quote!()
    } else {
        quote!(#[allow(clippy::infallible_try_from)])
    };

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics #domain::AggregateId for #name #ty_generics #where_clause {
            type Error = #error_ty;

            fn value(&self) -> #uuid::Uuid {
                self.0
            }

            fn try_from_uuid(value: #uuid::Uuid) -> ::std::result::Result<Self, Self::Error> {
                #validate_stmt
                Ok(Self(value))
            }
        }

        #[automatically_derived]
        impl #impl_generics ::std::convert::From<#name #ty_generics> for #uuid::Uuid #where_clause {
            fn from(value: #name #ty_generics) -> Self {
                value.value()
            }
        }

        #[automatically_derived]
        impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.value())
            }
        }

        #[automatically_derived]
        #try_from_allow
        impl #impl_generics ::std::convert::TryFrom<#uuid::Uuid> for #name #ty_generics #where_clause {
            type Error = #error_ty;

            fn try_from(value: #uuid::Uuid) -> ::std::result::Result<Self, Self::Error> {
                <Self as #domain::AggregateId>::try_from_uuid(value)
            }
        }

        const _: () = {
            fn _assert_inner_is_uuid(value: #inner_ty) -> #uuid::Uuid {
                value
            }
        };
    };

    Ok(expanded)
}

fn extract_inner_uuid_ty(data: &Data) -> Result<Type> {
    let fields = match data {
        Data::Struct(data) => &data.fields,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "`AggregateId` can only be derived for structs",
            ));
        }
    };

    let unnamed = match fields {
        Fields::Unnamed(fields) => &fields.unnamed,
        _ => {
            return Err(syn::Error::new(
                fields.span(),
                "`AggregateId` requires a tuple struct with one `Uuid` field",
            ));
        }
    };

    if unnamed.len() != 1 {
        return Err(syn::Error::new(
            unnamed.span(),
            "`AggregateId` requires a tuple struct with one `Uuid` field",
        ));
    }

    let ty = unnamed[0].ty.clone();
    let Type::Path(TypePath { path, .. }) = &ty else {
        return Err(syn::Error::new(
            ty.span(),
            "`AggregateId` requires a `Uuid` field",
        ));
    };

    let last = path
        .segments
        .last()
        .ok_or_else(|| syn::Error::new(ty.span(), "`AggregateId` requires a `Uuid` field"))?;

    if last.ident != "Uuid" {
        return Err(syn::Error::new(
            last.ident.span(),
            "`AggregateId` requires a `Uuid` field",
        ));
    }

    Ok(ty)
}
