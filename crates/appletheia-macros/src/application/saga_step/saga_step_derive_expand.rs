use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Result};

use crate::utils::crate_path::resolve_application_path;

pub(crate) fn expand_saga_step_derive(input: DeriveInput) -> Result<TokenStream> {
    if !matches!(input.data, Data::Enum(_)) {
        return Err(syn::Error::new(
            input.span(),
            "`SagaStep` can only be derived for enums",
        ));
    }

    let application = resolve_application_path()?;
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #application::saga::SagaStep for #name #ty_generics #where_clause {}
    })
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::expand_saga_step_derive;

    #[test]
    fn rejects_structs() {
        let error = expand_saga_step_derive(parse_quote!(
            struct InvalidStep;
        ))
        .expect_err("a step must be an enum");
        assert_eq!(
            error.to_string(),
            "`SagaStep` can only be derived for enums"
        );
    }

    #[test]
    fn rejects_unions() {
        let error = expand_saga_step_derive(parse_quote!(union InvalidStep { value: u32 }))
            .expect_err("a step must be an enum");
        assert_eq!(
            error.to_string(),
            "`SagaStep` can only be derived for enums"
        );
    }
}
