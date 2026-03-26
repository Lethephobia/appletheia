mod command;

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
