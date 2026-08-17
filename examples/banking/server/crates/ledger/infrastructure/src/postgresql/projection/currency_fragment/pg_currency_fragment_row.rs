use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_ledger_application::{
    CurrencyFragment, CurrencyFragmentWriterError, FragmentOwner, MaterializedCurrencyStatus,
};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{
    CurrencyDecimals, CurrencyDescription, CurrencyId, CurrencyName, CurrencySymbol,
    MintAccountAddress,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::postgresql::pg_currency_image_ref_columns::PgCurrencyImageRefColumns;

#[derive(Debug, sqlx::FromRow)]
pub struct PgCurrencyFragmentRow {
    pub id: Uuid,
    pub owner_type: String,
    pub owner_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub decimals: i16,
    pub description: Option<String>,
    pub image_type: Option<String>,
    pub image_object_name: Option<String>,
    pub image_external_url: Option<String>,
    pub mint_account_address: Option<String>,
    pub supply: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl PgCurrencyFragmentRow {
    pub fn try_into_fragment(
        self,
        owner: FragmentOwner,
    ) -> Result<CurrencyFragment, CurrencyFragmentWriterError> {
        let row = self;
        let status = match row.status.as_str() {
            "provisioning" => MaterializedCurrencyStatus::Provisioning,
            "active" => MaterializedCurrencyStatus::Active,
            "inactive" => MaterializedCurrencyStatus::Inactive,
            "provisioning_failed" => MaterializedCurrencyStatus::ProvisioningFailed,
            _ => return Err(persistence_message("unknown currency fragment status")),
        };
        let decimals = u8::try_from(row.decimals).map_err(persistence_error)?;

        Ok(CurrencyFragment {
            id: CurrencyId::try_from_uuid(row.id).map_err(persistence_error)?,
            owner,
            symbol: CurrencySymbol::try_from(row.symbol).map_err(persistence_error)?,
            name: CurrencyName::try_from(row.name).map_err(persistence_error)?,
            decimals: CurrencyDecimals::new(decimals),
            description: row
                .description
                .map(CurrencyDescription::try_from)
                .transpose()
                .map_err(persistence_error)?,
            image: PgCurrencyImageRefColumns {
                image_type: row.image_type,
                object_name: row.image_object_name,
                external_url: row.image_external_url,
            }
            .into_image()
            .map_err(persistence_error)?,
            mint_account_address: row
                .mint_account_address
                .map(MintAccountAddress::try_from)
                .transpose()
                .map_err(persistence_error)?,
            supply: amount(row.supply)?,
            status,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(persistence_error)?,
                EventId::try_from(row.updated_event_id).map_err(persistence_error)?,
            ),
        })
    }
}

fn amount(value: String) -> Result<CurrencyAmount, CurrencyFragmentWriterError> {
    let parsed = value.parse::<u128>().map_err(persistence_error)?;
    Ok(CurrencyAmount::new(parsed))
}

fn persistence_error(
    error: impl std::error::Error + Send + Sync + 'static,
) -> CurrencyFragmentWriterError {
    CurrencyFragmentWriterError::Persistence(Box::new(error))
}

fn persistence_message(message: &'static str) -> CurrencyFragmentWriterError {
    CurrencyFragmentWriterError::Persistence(Box::new(std::io::Error::other(message)))
}
