use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_ledger_application::{
    WalletBookmarkFragment, WalletBookmarkFragmentWriterError,
};
use banking_iam_domain::{OrganizationId, UserId};
use banking_ledger_domain::core::TokenAccountOwnerAddress;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId, WalletBookmarkOwner,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct PgWalletBookmarkFragmentRow {
    pub wallet_bookmark_id: Uuid,
    pub owner_type: String,
    pub owner_id: Uuid,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub token_account_owner_address: String,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgWalletBookmarkFragmentRow> for WalletBookmarkFragment {
    type Error = WalletBookmarkFragmentWriterError;

    fn try_from(row: PgWalletBookmarkFragmentRow) -> Result<Self, Self::Error> {
        Ok(WalletBookmarkFragment {
            wallet_bookmark_id: WalletBookmarkId::try_from_uuid(row.wallet_bookmark_id)
                .map_err(persistence_error)?,
            owner: match row.owner_type.as_str() {
                "user" => WalletBookmarkOwner::User(
                    UserId::try_from_uuid(row.owner_id).map_err(persistence_error)?,
                ),
                "organization" => WalletBookmarkOwner::Organization(
                    OrganizationId::try_from_uuid(row.owner_id).map_err(persistence_error)?,
                ),
                _ => return Err(persistence_message("unknown wallet bookmark owner type")),
            },
            display_name: row
                .display_name
                .map(WalletBookmarkDisplayName::try_from)
                .transpose()
                .map_err(persistence_error)?,
            description: row
                .description
                .map(WalletBookmarkDescription::try_from)
                .transpose()
                .map_err(persistence_error)?,
            token_account_owner_address: TokenAccountOwnerAddress::try_from(
                row.token_account_owner_address,
            )
            .map_err(persistence_error)?,
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(persistence_error)?,
                EventId::try_from(row.updated_event_id).map_err(persistence_error)?,
            ),
        })
    }
}

fn persistence_message(message: &'static str) -> WalletBookmarkFragmentWriterError {
    WalletBookmarkFragmentWriterError::Persistence(Box::new(std::io::Error::other(message)))
}

fn persistence_error(
    error: impl std::error::Error + Send + Sync + 'static,
) -> WalletBookmarkFragmentWriterError {
    WalletBookmarkFragmentWriterError::Persistence(Box::new(error))
}
