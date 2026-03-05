use syn::spanned::Spanned;
use syn::{Attribute, Ident, LitStr, Result, Type};

#[derive(Debug)]
pub(crate) struct AggregateArgs {
    pub(crate) aggregate_type: LitStr,
    pub(crate) core_field: Option<Ident>,
    pub(crate) error: Type,
}

impl AggregateArgs {
    pub(crate) fn from_attrs(attrs: &[Attribute]) -> Result<Self> {
        let mut aggregate_type: Option<LitStr> = None;
        let mut core_field: Option<Ident> = None;
        let mut error: Option<Type> = None;

        for attr in attrs {
            if !attr.path().is_ident("aggregate_derive") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("type") {
                    let value: LitStr = meta.value()?.parse()?;
                    if aggregate_type.is_some() {
                        return Err(syn::Error::new(meta.path.span(), "duplicate `type`"));
                    }
                    aggregate_type = Some(value);
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

                if meta.path.is_ident("core") {
                    let value_stream = meta.value()?;
                    let value_expr: syn::Expr = value_stream.parse()?;
                    let value_ident = match value_expr {
                        syn::Expr::Path(expr) => {
                            expr.path.get_ident().cloned().ok_or_else(|| {
                                syn::Error::new(expr.span(), "`core` must be an ident")
                            })?
                        }
                        syn::Expr::Lit(expr) => {
                            let syn::Lit::Str(value) = expr.lit else {
                                return Err(syn::Error::new(
                                    expr.span(),
                                    "`core` must be an ident",
                                ));
                            };
                            Ident::new(&value.value(), value.span())
                        }
                        _ => {
                            return Err(syn::Error::new(
                                value_expr.span(),
                                "`core` must be an ident",
                            ));
                        }
                    };
                    if core_field.is_some() {
                        return Err(syn::Error::new(meta.path.span(), "duplicate `core`"));
                    }
                    core_field = Some(value_ident);
                    return Ok(());
                }

                Err(syn::Error::new(
                    meta.path.span(),
                    "unsupported key (expected `type`, `error`, or `core`)",
                ))
            })?;
        }

        let aggregate_type = aggregate_type.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing `#[aggregate(type = \"...\")]` (or `#[aggregate_derive(type = \"...\")]` when using `#[derive(Aggregate)]` directly)",
            )
        })?;

        let error = error.ok_or_else(|| {
            syn::Error::new(
                aggregate_type.span(),
                "missing `#[aggregate(error = ...)]` (or `#[aggregate_derive(error = ...)]` when using `#[derive(Aggregate)]` directly)",
            )
        })?;

        Ok(Self {
            aggregate_type,
            core_field,
            error,
        })
    }
}
