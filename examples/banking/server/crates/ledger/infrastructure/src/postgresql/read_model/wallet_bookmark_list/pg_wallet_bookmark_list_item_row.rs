use appletheia::domain::{AggregateId, EventOccurredAt};
use banking_iam_domain::{OrganizationId, UserId};
use banking_ledger_application::WalletBookmarkListItem;
use banking_ledger_domain::core::TokenOwnerAddress;
use banking_ledger_domain::wallet_bookmark::{
    WalletBookmarkDescription, WalletBookmarkDisplayName, WalletBookmarkId, WalletBookmarkOwner,
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use super::pg_wallet_bookmark_list_item_row_error::PgWalletBookmarkListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgWalletBookmarkListItemRow {
    pub wallet_bookmark_id: Uuid,
    pub owner_type: String,
    pub owner_id: Uuid,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub token_owner_address: String,
    pub created_at: DateTime<Utc>,
}

impl PgWalletBookmarkListItemRow {
    fn owner(&self) -> Result<WalletBookmarkOwner, PgWalletBookmarkListItemRowError> {
        match self.owner_type.as_str() {
            "user" => Ok(WalletBookmarkOwner::User(
                UserId::try_from_uuid(self.owner_id).map_err(|error| {
                    PgWalletBookmarkListItemRowError::InvalidUserOwnerId(Box::new(error))
                })?,
            )),
            "organization" => Ok(WalletBookmarkOwner::Organization(
                OrganizationId::try_from_uuid(self.owner_id).map_err(|error| {
                    PgWalletBookmarkListItemRowError::InvalidOrganizationOwnerId(Box::new(error))
                })?,
            )),
            value => Err(PgWalletBookmarkListItemRowError::UnknownOwnerType(
                value.to_owned(),
            )),
        }
    }
}

impl TryFrom<PgWalletBookmarkListItemRow> for WalletBookmarkListItem {
    type Error = PgWalletBookmarkListItemRowError;

    fn try_from(row: PgWalletBookmarkListItemRow) -> Result<Self, Self::Error> {
        let owner = row.owner()?;

        Ok(Self {
            wallet_bookmark_id: WalletBookmarkId::try_from_uuid(row.wallet_bookmark_id).map_err(
                |error| PgWalletBookmarkListItemRowError::InvalidWalletBookmarkId(Box::new(error)),
            )?,
            owner,
            display_name: row
                .display_name
                .map(WalletBookmarkDisplayName::try_from)
                .transpose()
                .map_err(|error| {
                    PgWalletBookmarkListItemRowError::InvalidDisplayName(Box::new(error))
                })?,
            description: row
                .description
                .map(WalletBookmarkDescription::try_from)
                .transpose()
                .map_err(|error| {
                    PgWalletBookmarkListItemRowError::InvalidDescription(Box::new(error))
                })?,
            token_owner_address: serde_json::from_str::<TokenOwnerAddress>(
                &row.token_owner_address,
            )
            .map_err(|error| {
                PgWalletBookmarkListItemRowError::InvalidTokenAccountOwnerAddress(Box::new(error))
            })?,
            created_at: EventOccurredAt::from(row.created_at),
        })
    }
}
