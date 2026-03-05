mod aggregate_attribute_expand;
mod aggregate_derive_args;
mod aggregate_derive_expand;

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
