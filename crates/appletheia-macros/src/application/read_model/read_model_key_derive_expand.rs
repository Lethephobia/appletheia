use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Result};

use crate::utils::crate_path::resolve_application_path;

pub(crate) fn expand_read_model_key_derive(
    input: DeriveInput,
    trait_name: &str,
) -> Result<TokenStream> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input,
            "read model keys require an enum",
        ));
    };
    let application = resolve_application_path()?;
    let trait_ident = format_ident!("{trait_name}");
    let error_ident = format_ident!("{trait_name}Error");
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut arms = Vec::new();

    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                variant,
                "read model keys require unit variants",
            ));
        }
        let variant_ident = &variant.ident;
        let variant_name = variant_ident.to_string();
        let mut key = String::new();
        for (index, character) in variant_name.trim_start_matches("r#").chars().enumerate() {
            if character.is_ascii_uppercase() {
                if index > 0 {
                    key.push('_');
                }
                key.push(character.to_ascii_lowercase());
            } else {
                key.push(character);
            }
        }
        arms.push(quote! {
            Self::#variant_ident => Ok(#application::read_model::ReadModelResourceKey::new(
                ::std::string::String::from(#key)
            )?),
        });
    }

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #application::read_model::#trait_ident for #name #ty_generics #where_clause {
            fn try_into_resource_key(self) -> ::core::result::Result<
                #application::read_model::ReadModelResourceKey,
                #application::read_model::#error_ident,
            > {
                match self { #(#arms)* }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::expand_read_model_key_derive;
    use syn::parse_quote;

    #[test]
    fn rejects_non_enum_and_payload_variants() {
        for input in [
            parse_quote!(
                struct Key;
            ),
            parse_quote!(
                enum Key {
                    Name(String),
                }
            ),
            parse_quote!(
                enum Key {
                    Name { value: String },
                }
            ),
        ] {
            assert!(expand_read_model_key_derive(input, "ReadModelAttributeKey").is_err());
        }
    }

    #[test]
    fn supports_empty_relationship_keys() {
        assert!(
            expand_read_model_key_derive(
                parse_quote!(
                    enum Key {}
                ),
                "ReadModelRelationshipKey",
            )
            .is_ok()
        );
    }
}
