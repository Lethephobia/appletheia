use banking_ledger_domain::currency::{
    CurrencyImageObjectName, CurrencyImageRef, CurrencyImageUrl,
};

use super::pg_currency_image_ref_columns_error::PgCurrencyImageRefColumnsError;

/// PostgreSQL columns that encode a currency image reference.
#[derive(Debug)]
pub(crate) struct PgCurrencyImageRefColumns {
    pub image_type: Option<String>,
    pub object_name: Option<String>,
    pub external_url: Option<String>,
}

impl PgCurrencyImageRefColumns {
    const EXTERNAL_URL: &'static str = "external_url";
    const OBJECT_NAME: &'static str = "object_name";

    pub(crate) fn from_image(
        image: Option<&CurrencyImageRef>,
    ) -> (Option<&'static str>, Option<&str>, Option<&str>) {
        match image {
            Some(CurrencyImageRef::ObjectName(object_name)) => {
                (Some(Self::OBJECT_NAME), Some(object_name.value()), None)
            }
            Some(CurrencyImageRef::ExternalUrl(url)) => {
                (Some(Self::EXTERNAL_URL), None, Some(url.value().as_str()))
            }
            None => (None, None, None),
        }
    }

    pub(crate) fn into_image(
        self,
    ) -> Result<Option<CurrencyImageRef>, PgCurrencyImageRefColumnsError> {
        match (
            self.image_type.as_deref(),
            self.object_name,
            self.external_url,
        ) {
            (None, None, None) => Ok(None),
            (Some(Self::OBJECT_NAME), Some(object_name), None) => {
                let object_name = CurrencyImageObjectName::try_from(object_name)
                    .map_err(|error| PgCurrencyImageRefColumnsError::ObjectName(Box::new(error)))?;
                Ok(Some(CurrencyImageRef::object_name(object_name)))
            }
            (Some(Self::EXTERNAL_URL), None, Some(url)) => {
                let url = CurrencyImageUrl::try_from(url).map_err(|error| {
                    PgCurrencyImageRefColumnsError::ExternalUrl(Box::new(error))
                })?;
                Ok(Some(CurrencyImageRef::external_url(url)))
            }
            (Some(value), _, _) if value != Self::OBJECT_NAME && value != Self::EXTERNAL_URL => {
                Err(PgCurrencyImageRefColumnsError::UnknownType(
                    value.to_owned(),
                ))
            }
            _ => Err(PgCurrencyImageRefColumnsError::InconsistentColumns),
        }
    }
}
