use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_iam_domain::{OrganizationId, UserId};
use banking_ledger_application::{
    AccountFragment, AccountFragmentWriterError, MaterializedAccountStatus,
};
use banking_ledger_domain::account::{AccountId, AccountName, AccountOwner};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct PgAccountFragmentRow {
    pub id: Uuid,
    pub owner_type: String,
    pub owner_id: Uuid,
    pub name: String,
    pub currency_id: Uuid,
    pub balance: String,
    pub reserved_balance: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgAccountFragmentRow> for AccountFragment {
    type Error = AccountFragmentWriterError;

    fn try_from(row: PgAccountFragmentRow) -> Result<Self, Self::Error> {
        let status = match row.status.as_str() {
            "active" => MaterializedAccountStatus::Active,
            "frozen" => MaterializedAccountStatus::Frozen,
            _ => return Err(persistence_message("unknown account fragment status")),
        };

        Ok(AccountFragment {
            id: AccountId::try_from_uuid(row.id).map_err(persistence_error)?,
            owner: match row.owner_type.as_str() {
                "user" => AccountOwner::User(
                    UserId::try_from_uuid(row.owner_id).map_err(persistence_error)?,
                ),
                "organization" => AccountOwner::Organization(
                    OrganizationId::try_from_uuid(row.owner_id).map_err(persistence_error)?,
                ),
                _ => return Err(persistence_message("unknown account fragment owner type")),
            },
            name: AccountName::try_from(row.name).map_err(persistence_error)?,
            currency_id: CurrencyId::try_from_uuid(row.currency_id).map_err(persistence_error)?,
            balance: amount(row.balance)?,
            reserved_balance: amount(row.reserved_balance)?,
            status,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(persistence_error)?,
                EventId::try_from(row.updated_event_id).map_err(persistence_error)?,
            ),
        })
    }
}

fn amount(value: String) -> Result<CurrencyAmount, AccountFragmentWriterError> {
    let parsed = value.parse::<u128>().map_err(persistence_error)?;
    Ok(CurrencyAmount::new(parsed))
}

fn persistence_error(
    error: impl std::error::Error + Send + Sync + 'static,
) -> AccountFragmentWriterError {
    AccountFragmentWriterError::Persistence(Box::new(error))
}

fn persistence_message(message: &'static str) -> AccountFragmentWriterError {
    AccountFragmentWriterError::Persistence(Box::new(std::io::Error::other(message)))
}
