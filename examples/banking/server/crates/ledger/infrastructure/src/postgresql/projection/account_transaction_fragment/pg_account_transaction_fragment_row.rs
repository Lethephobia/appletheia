use appletheia::application::read_model::ReadModelObservation;
use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
use banking_ledger_application::{
    AccountTransactionDirection, AccountTransactionFragment, AccountTransactionFragmentKind,
    AccountTransactionFragmentWriterError, AccountTransactionId, AccountTransactionStatus,
};
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::transfer::TransferId;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct PgAccountTransactionFragmentRow {
    pub transaction_id: Uuid,
    pub transfer_id: Option<Uuid>,
    pub account_id: Uuid,
    pub counterparty_account_id: Option<Uuid>,
    pub amount: String,
    pub direction: String,
    pub kind: String,
    pub status: String,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub source_event_id: Uuid,
    pub updated_event_id: Uuid,
}

impl TryFrom<PgAccountTransactionFragmentRow> for AccountTransactionFragment {
    type Error = AccountTransactionFragmentWriterError;

    fn try_from(row: PgAccountTransactionFragmentRow) -> Result<Self, Self::Error> {
        let direction = match row.direction.as_str() {
            "incoming" => AccountTransactionDirection::Incoming,
            "outgoing" => AccountTransactionDirection::Outgoing,
            _ => return Err(persistence_message("unknown account transaction direction")),
        };
        let kind = match row.kind.as_str() {
            "deposit" => AccountTransactionFragmentKind::Deposit,
            "withdrawal" => AccountTransactionFragmentKind::Withdrawal,
            "transfer" => AccountTransactionFragmentKind::Transfer,
            "currency_issuance" => AccountTransactionFragmentKind::CurrencyIssuance,
            _ => return Err(persistence_message("unknown account transaction kind")),
        };
        let status = match row.status.as_str() {
            "pending" => AccountTransactionStatus::Pending,
            "completed" => AccountTransactionStatus::Completed,
            "failed" => AccountTransactionStatus::Failed,
            "requires_review" => AccountTransactionStatus::RequiresReview,
            _ => return Err(persistence_message("unknown account transaction status")),
        };

        Ok(AccountTransactionFragment {
            transaction_id: AccountTransactionId::from(row.transaction_id),
            transfer_id: row
                .transfer_id
                .map(TransferId::try_from_uuid)
                .transpose()
                .map_err(persistence_error)?,
            account_id: AccountId::try_from_uuid(row.account_id).map_err(persistence_error)?,
            counterparty_account_id: row
                .counterparty_account_id
                .map(AccountId::try_from_uuid)
                .transpose()
                .map_err(persistence_error)?,
            amount: amount(row.amount)?,
            direction,
            kind,
            status,
            occurred_at: EventOccurredAt::from(row.occurred_at),
            created_at: EventOccurredAt::from(row.created_at),
            observation: ReadModelObservation::new(
                EventId::try_from(row.source_event_id).map_err(persistence_error)?,
                EventId::try_from(row.updated_event_id).map_err(persistence_error)?,
            ),
        })
    }
}

fn amount(value: String) -> Result<CurrencyAmount, AccountTransactionFragmentWriterError> {
    let parsed = value.parse::<u128>().map_err(persistence_error)?;
    Ok(CurrencyAmount::new(parsed))
}

fn persistence_error(
    error: impl std::error::Error + Send + Sync + 'static,
) -> AccountTransactionFragmentWriterError {
    AccountTransactionFragmentWriterError::Persistence(Box::new(error))
}

fn persistence_message(message: &'static str) -> AccountTransactionFragmentWriterError {
    AccountTransactionFragmentWriterError::Persistence(Box::new(std::io::Error::other(message)))
}
