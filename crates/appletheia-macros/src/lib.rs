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

#[proc_macro_derive(AggregateId, attributes(aggregate_id_derive))]
pub fn aggregate_id_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    domain::aggregate::aggregate_id_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn aggregate_id(attr: TokenStream, item: TokenStream) -> TokenStream {
    domain::aggregate::aggregate_id_attribute(attr, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(AggregateState, attributes(aggregate_state_derive))]
pub fn aggregate_state_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    domain::aggregate::aggregate_state_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn aggregate_state(attr: TokenStream, item: TokenStream) -> TokenStream {
    domain::aggregate::aggregate_state_attribute(attr, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn unique_constraints(attr: TokenStream, item: TokenStream) -> TokenStream {
    domain::aggregate::unique_constraints_attribute(attr, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(EventPayload, attributes(event_payload_derive))]
pub fn event_payload_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    domain::event::event_payload_derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn event_payload(attr: TokenStream, item: TokenStream) -> TokenStream {
    domain::event::event_payload_attribute(attr, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
