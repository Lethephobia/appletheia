use proc_macro_crate::{FoundCrate, crate_name};
use syn::{Ident, Path, Result, parse_quote};

fn resolve_crate_root(package: &str) -> Result<Path> {
    match crate_name(package) {
        Ok(FoundCrate::Itself) => Ok(parse_quote!(crate)),
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, proc_macro2::Span::call_site());
            Ok(parse_quote!(::#ident))
        }
        Err(err) => Err(syn::Error::new(proc_macro2::Span::call_site(), err)),
    }
}

pub(crate) fn resolve_macros_root() -> Result<Path> {
    if let Ok(macros) = resolve_crate_root("appletheia-macros") {
        return Ok(macros);
    }

    resolve_crate_root("appletheia")
}

pub(crate) fn resolve_domain_path() -> Result<Path> {
    if let Ok(appletheia) = resolve_crate_root("appletheia") {
        return Ok(parse_quote!(#appletheia::domain));
    }

    let appletheia_domain = resolve_crate_root("appletheia-domain")?;
    Ok(appletheia_domain)
}

#[allow(dead_code)]
pub(crate) fn resolve_application_path() -> Result<Path> {
    if let Ok(appletheia) = resolve_crate_root("appletheia") {
        return Ok(parse_quote!(#appletheia::application));
    }

    let appletheia_application = resolve_crate_root("appletheia-application")?;
    Ok(appletheia_application)
}
