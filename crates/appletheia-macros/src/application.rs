mod command;
mod relations;

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

pub(crate) fn relations_attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> Result<TokenStream> {
    relations::relations_attribute_expand::expand_relations_attribute(attr, item)
}

pub(crate) fn relations_derive(input: DeriveInput) -> Result<TokenStream> {
    let args = relations::relations_derive_args::RelationsDeriveArgs::from_attrs(&input.attrs)?;
    relations::relations_derive_expand::expand_relations_derive(input, args)
}
