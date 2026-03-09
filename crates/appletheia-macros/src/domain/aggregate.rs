mod aggregate_attribute_expand;
mod aggregate_derive_args;
mod aggregate_derive_expand;
mod aggregate_id_attribute_expand;
mod aggregate_id_derive_args;
mod aggregate_id_derive_expand;
mod aggregate_state_attribute_expand;
mod aggregate_state_derive_args;
mod aggregate_state_derive_expand;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Result};

pub(crate) fn aggregate_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    aggregate_attribute_expand::expand_aggregate_attribute(attr, item)
}

pub(crate) fn aggregate_derive(input: DeriveInput) -> Result<TokenStream> {
    let args = aggregate_derive_args::AggregateArgs::from_attrs(&input.attrs)?;
    aggregate_derive_expand::expand_aggregate(input, args)
}

pub(crate) fn aggregate_id_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    aggregate_id_attribute_expand::expand_aggregate_id_attribute(attr, item)
}

pub(crate) fn aggregate_id_derive(input: DeriveInput) -> Result<TokenStream> {
    let args = aggregate_id_derive_args::AggregateIdArgs::from_attrs(&input.attrs)?;
    aggregate_id_derive_expand::expand_aggregate_id(input, args)
}

pub(crate) fn aggregate_state_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    aggregate_state_attribute_expand::expand_aggregate_state_attribute(attr, item)
}

pub(crate) fn aggregate_state_derive(input: DeriveInput) -> Result<TokenStream> {
    let args = aggregate_state_derive_args::AggregateStateArgs::from_attrs(&input.attrs)?;
    aggregate_state_derive_expand::expand_aggregate_state(input, args)
}
