use syn::spanned::Spanned;
use syn::{Attribute, Ident, Result, Type};

#[derive(Debug)]
pub(crate) struct AggregateStateDeriveArgs {
    pub(crate) id_field: Ident,
    pub(crate) error: Type,
}

impl AggregateStateDeriveArgs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut id_field: Option<Ident> = None;
        let mut error: Option<Type> = None;

        for attr in attrs {
            if !attr.path().is_ident("aggregate_state_derive") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("id") {
                    let value_stream = meta.value()?;
                    let value_expr: syn::Expr = value_stream.parse()?;
                    let value_ident = match value_expr {
                        syn::Expr::Path(expr) => {
                            expr.path.get_ident().cloned().ok_or_else(|| {
                                syn::Error::new(expr.span(), "`id` must be an ident")
                            })?
                        }
                        syn::Expr::Lit(expr) => {
                            let syn::Lit::Str(value) = expr.lit else {
                                return Err(syn::Error::new(expr.span(), "`id` must be an ident"));
                            };
                            Ident::new(&value.value(), value.span())
                        }
                        _ => {
                            return Err(syn::Error::new(
                                value_expr.span(),
                                "`id` must be an ident",
                            ));
                        }
                    };
                    if id_field.is_some() {
                        return Err(syn::Error::new(meta.path.span(), "duplicate `id`"));
                    }
                    id_field = Some(value_ident);
                    return Ok(());
                }

                if meta.path.is_ident("error") {
                    let value: Type = meta.value()?.parse()?;
                    if error.is_some() {
                        return Err(syn::Error::new(meta.path.span(), "duplicate `error`"));
                    }
                    error = Some(value);
                    return Ok(());
                }

                Err(syn::Error::new(
                    meta.path.span(),
                    "unsupported key (expected `id` or `error`)",
                ))
            })?;
        }

        let id_field = id_field.unwrap_or_else(|| Ident::new("id", proc_macro2::Span::call_site()));
        let error = error.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing `#[aggregate_state(error = ...)]` (or `#[aggregate_state_derive(error = ...)]` when using `#[derive(AggregateState)]` directly)",
            )
        })?;

        Ok(Self { id_field, error })
    }
}
