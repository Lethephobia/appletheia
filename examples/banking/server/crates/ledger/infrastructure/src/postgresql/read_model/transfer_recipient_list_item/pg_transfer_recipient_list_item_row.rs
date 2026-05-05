use appletheia::domain::{AggregateId, EventOccurredAt};
use banking_iam_domain::{UserDisplayName, UserId, UserPictureRef, Username};
use banking_ledger_application::{
    TransferRecipientListItem, TransferRecipientListItemAccount, TransferRecipientListItemCurrency,
};
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};
use sqlx::types::chrono::{DateTime, Utc};

use super::pg_transfer_recipient_list_item_row_error::PgTransferRecipientListItemRowError;

#[derive(Debug, sqlx::FromRow)]
pub struct PgTransferRecipientListItemRow {
    pub user_id: uuid::Uuid,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub picture: Option<sqlx::types::Json<UserPictureRef>>,
    pub user_created_at: DateTime<Utc>,
    pub account_id: uuid::Uuid,
    pub currency_id: uuid::Uuid,
    pub currency_symbol: String,
    pub currency_name: String,
    pub currency_decimals: i16,
}

impl PgTransferRecipientListItemRow {
    pub fn user_id(&self) -> Result<UserId, PgTransferRecipientListItemRowError> {
        UserId::try_from_uuid(self.user_id)
            .map_err(|error| PgTransferRecipientListItemRowError::InvalidUserId(Box::new(error)))
    }

    fn optional_username(
        value: Option<String>,
    ) -> Result<Option<Username>, PgTransferRecipientListItemRowError> {
        value
            .map(Username::try_from)
            .transpose()
            .map_err(|error| PgTransferRecipientListItemRowError::InvalidUsername(Box::new(error)))
    }

    fn optional_user_display_name(
        value: Option<String>,
    ) -> Result<Option<UserDisplayName>, PgTransferRecipientListItemRowError> {
        value
            .map(UserDisplayName::try_from)
            .transpose()
            .map_err(|error| {
                PgTransferRecipientListItemRowError::InvalidUserDisplayName(Box::new(error))
            })
    }

    pub fn list_item(
        &self,
    ) -> Result<TransferRecipientListItem, PgTransferRecipientListItemRowError> {
        Ok(TransferRecipientListItem {
            user_id: self.user_id()?,
            username: Self::optional_username(self.username.clone())?,
            display_name: Self::optional_user_display_name(self.display_name.clone())?,
            picture: self.picture.clone().map(|value| value.0),
            accounts: Vec::new(),
            created_at: EventOccurredAt::from(self.user_created_at),
        })
    }

    pub fn account(
        &self,
    ) -> Result<TransferRecipientListItemAccount, PgTransferRecipientListItemRowError> {
        let currency_decimals = u8::try_from(self.currency_decimals).map_err(|error| {
            PgTransferRecipientListItemRowError::InvalidCurrencyDecimals(Box::new(error))
        })?;

        Ok(TransferRecipientListItemAccount {
            account_id: AccountId::try_from_uuid(self.account_id).map_err(|error| {
                PgTransferRecipientListItemRowError::InvalidAccountId(Box::new(error))
            })?,
            currency: TransferRecipientListItemCurrency {
                id: CurrencyId::try_from_uuid(self.currency_id).map_err(|error| {
                    PgTransferRecipientListItemRowError::InvalidCurrencyId(Box::new(error))
                })?,
                symbol: CurrencySymbol::try_from(self.currency_symbol.clone()).map_err(
                    |error| {
                        PgTransferRecipientListItemRowError::InvalidCurrencySymbol(Box::new(error))
                    },
                )?,
                name: CurrencyName::try_from(self.currency_name.clone()).map_err(|error| {
                    PgTransferRecipientListItemRowError::InvalidCurrencyName(Box::new(error))
                })?,
                decimals: CurrencyDecimals::new(currency_decimals),
            },
        })
    }
}
