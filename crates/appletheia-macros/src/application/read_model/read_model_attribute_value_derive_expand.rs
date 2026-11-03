use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result, parse_quote};

use crate::utils::crate_path::{
    resolve_application_path, resolve_serde_json_root, resolve_serde_root,
};

pub(crate) fn expand_read_model_attribute_value_derive(input: DeriveInput) -> Result<TokenStream> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input,
            "attribute values require an enum",
        ));
    };
    let application = resolve_application_path()?;
    let serde_json = resolve_serde_json_root()?;
    let serde = resolve_serde_root()?;
    let name = &input.ident;
    let mut generics = input.generics.clone();
    let mut arms = Vec::new();

    for variant in &data.variants {
        let Fields::Unnamed(fields) = &variant.fields else {
            return Err(syn::Error::new_spanned(
                variant,
                "attribute values require single-field tuple variants",
            ));
        };
        if fields.unnamed.len() != 1 {
            return Err(syn::Error::new_spanned(
                variant,
                "attribute values require single-field tuple variants",
            ));
        }
        for field in &fields.unnamed {
            let ty = &field.ty;
            generics
                .make_where_clause()
                .predicates
                .push(parse_quote!(#ty: #serde::Serialize));
        }
        let variant_ident = &variant.ident;
        arms.push(quote! {
            Self::#variant_ident(value) => Ok(#application::read_model::ReadModelResourceAttributeValue::from(
                #serde_json::to_value(value)?
            )),
        });
    }

    generics
        .make_where_clause()
        .predicates
        .push(parse_quote!(Self: Send + Sync));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #application::read_model::ReadModelAttributeValue for #name #ty_generics #where_clause {
            fn try_into_resource_attribute_value(self) -> ::core::result::Result<
                #application::read_model::ReadModelResourceAttributeValue,
                #application::read_model::ReadModelAttributeValueError,
            > {
                match self { #(#arms)* }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::expand_read_model_attribute_value_derive;
    use syn::parse_quote;

    #[test]
    fn rejects_invalid_value_shapes() {
        for input in [
            parse_quote!(
                struct Value;
            ),
            parse_quote!(
                enum Value {
                    Name,
                }
            ),
            parse_quote!(
                enum Value {
                    Name(String, String),
                }
            ),
            parse_quote!(
                enum Value {
                    Name { value: String },
                }
            ),
        ] {
            assert!(expand_read_model_attribute_value_derive(input).is_err());
        }
    }
}
