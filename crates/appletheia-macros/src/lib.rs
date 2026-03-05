extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{DeriveInput, parse_macro_input};

mod application;
mod domain;
mod utils;

#[proc_macro_derive(Aggregate, attributes(aggregate_derive))]
pub fn aggregate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    domain::aggregate::aggregate_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn aggregate(attr: TokenStream, item: TokenStream) -> TokenStream {
    domain::aggregate::aggregate_attribute(attr, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
