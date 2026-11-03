mod command;
mod query;
mod read_model;
mod saga_step;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Result};

pub(crate) fn command_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    command::command_attribute_expand::expand_command_attribute(attr, item)
}

pub(crate) fn command_derive(input: DeriveInput) -> Result<TokenStream> {
    let args = command::command_derive_args::CommandDeriveArgs::from_attrs(&input.attrs)?;
    command::command_derive_expand::expand_command_derive(input, args)
}

pub(crate) fn query_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    query::query_attribute_expand::expand_query_attribute(attr, item)
}

pub(crate) fn query_derive(input: DeriveInput) -> Result<TokenStream> {
    let args = query::query_derive_args::QueryDeriveArgs::from_attrs(&input.attrs)?;
    query::query_derive_expand::expand_query_derive(input, args)
}

pub(crate) fn saga_step_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    saga_step::saga_step_attribute_expand::expand_saga_step_attribute(attr, item)
}

pub(crate) fn saga_step_derive(input: DeriveInput) -> Result<TokenStream> {
    saga_step::saga_step_derive_expand::expand_saga_step_derive(input)
}

pub(crate) fn read_model_attribute_key_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    read_model::read_model_attribute_expand::expand_read_model_attribute(
        attr.into(),
        item.into(),
        "ReadModelAttributeKey",
        true,
    )
}

pub(crate) fn read_model_attribute_key_derive(input: DeriveInput) -> Result<TokenStream> {
    read_model::read_model_key_derive_expand::expand_read_model_key_derive(
        input,
        "ReadModelAttributeKey",
    )
}

pub(crate) fn read_model_relationship_key_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    read_model::read_model_attribute_expand::expand_read_model_attribute(
        attr.into(),
        item.into(),
        "ReadModelRelationshipKey",
        true,
    )
}

pub(crate) fn read_model_relationship_key_derive(input: DeriveInput) -> Result<TokenStream> {
    read_model::read_model_key_derive_expand::expand_read_model_key_derive(
        input,
        "ReadModelRelationshipKey",
    )
}

pub(crate) fn read_model_attribute_value_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    read_model::read_model_attribute_expand::expand_read_model_attribute(
        attr.into(),
        item.into(),
        "ReadModelAttributeValue",
        false,
    )
}

pub(crate) fn read_model_attribute_value_derive(input: DeriveInput) -> Result<TokenStream> {
    read_model::read_model_attribute_value_derive_expand::expand_read_model_attribute_value_derive(
        input,
    )
}
