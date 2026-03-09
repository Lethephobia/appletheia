mod event_payload_attribute_expand;
mod event_payload_derive_args;
mod event_payload_derive_expand;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Result};

pub(crate) fn event_payload_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    event_payload_attribute_expand::expand_event_payload_attribute(attr, item)
}

pub(crate) fn event_payload_derive(input: DeriveInput) -> Result<TokenStream> {
    let args = event_payload_derive_args::EventPayloadDeriveArgs::from_attrs(&input.attrs)?;
    event_payload_derive_expand::expand_event_payload_derive(input, args)
}
