use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    Data, DeriveInput, Fields, GenericArgument, Ident, PathArguments, Result, Type, TypePath,
};

use crate::domain::aggregate::aggregate_derive_args::AggregateDeriveArgs;
use crate::utils::crate_path::resolve_domain_path;

pub(crate) fn expand_aggregate_derive(
    input: DeriveInput,
    args: AggregateDeriveArgs,
) -> Result<TokenStream> {
    let domain = resolve_domain_path()?;

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let core_field = args
        .core_field
        .unwrap_or_else(|| Ident::new("core", proc_macro2::Span::call_site()));

    let (state_ty, payload_ty) = extract_core_generics(&input.data, &core_field)?;

    let aggregate_type = args.aggregate_type;
    let error_ty = args.error;

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics #domain::Aggregate for #name #ty_generics #where_clause {
            type Id = <#state_ty as #domain::AggregateState>::Id;
            type State = #state_ty;
            type EventPayload = #payload_ty;
            type Error = #error_ty;

            const TYPE: #domain::AggregateType = #domain::AggregateType::new(#aggregate_type);

            fn core(&self) -> &#domain::AggregateCore<Self::State, Self::EventPayload> {
                &self.#core_field
            }

            fn core_mut(&mut self) -> &mut #domain::AggregateCore<Self::State, Self::EventPayload> {
                &mut self.#core_field
            }
        }
    };

    Ok(expanded)
}

fn extract_core_generics(data: &Data, core_field: &Ident) -> Result<(Type, Type)> {
    let fields = match data {
        Data::Struct(data) => &data.fields,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "`Aggregate` can only be derived for structs",
            ));
        }
    };

    let named_fields = match fields {
        Fields::Named(fields) => &fields.named,
        _ => {
            return Err(syn::Error::new(
                fields.span(),
                "`Aggregate` requires a struct with named fields",
            ));
        }
    };

    let core = named_fields.iter().find(|field| {
        field
            .ident
            .as_ref()
            .is_some_and(|ident| ident == core_field)
    });

    let core = core.ok_or_else(|| {
        syn::Error::new(
            named_fields.span(),
            format!("missing `{core_field}: AggregateCore<...>` field"),
        )
    })?;

    let Type::Path(TypePath { path, .. }) = &core.ty else {
        return Err(syn::Error::new(
            core.ty.span(),
            "`core` field must be `AggregateCore<State, Payload>`",
        ));
    };

    let last = path.segments.last().ok_or_else(|| {
        syn::Error::new(
            core.ty.span(),
            "`core` field must be `AggregateCore<State, Payload>`",
        )
    })?;

    if last.ident != "AggregateCore" {
        return Err(syn::Error::new(
            last.ident.span(),
            "`core` field must be `AggregateCore<State, Payload>`",
        ));
    }

    let PathArguments::AngleBracketed(args) = &last.arguments else {
        return Err(syn::Error::new(
            last.arguments.span(),
            "`AggregateCore` must have generic arguments: `AggregateCore<State, Payload>`",
        ));
    };

    let mut ty_args = args.args.iter().filter_map(|arg| match arg {
        GenericArgument::Type(ty) => Some(ty.clone()),
        _ => None,
    });

    let state_ty = ty_args.next().ok_or_else(|| {
        syn::Error::new(
            args.span(),
            "`AggregateCore` must be `AggregateCore<State, Payload>`",
        )
    })?;

    let payload_ty = ty_args.next().ok_or_else(|| {
        syn::Error::new(
            args.span(),
            "`AggregateCore` must be `AggregateCore<State, Payload>`",
        )
    })?;

    Ok((state_ty, payload_ty))
}
