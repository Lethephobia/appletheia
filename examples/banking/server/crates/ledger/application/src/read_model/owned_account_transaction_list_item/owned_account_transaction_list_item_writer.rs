use appletheia::application::event::EventSequence;
use appletheia::application::request_context::CorrelationId;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::{EventId, EventOccurredAt};
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{CurrencyDecimals, CurrencyId, CurrencyName, CurrencySymbol};
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use banking_ledger_domain::transfer::{TransferFailureReason, TransferId};

use super::{
    OwnedAccountTransactionListItemDirection, OwnedAccountTransactionListItemKind,
    OwnedAccountTransactionListItemStatus, OwnedAccountTransactionListItemWriterError,
};

/// Writes owned account transaction list read models.
#[allow(async_fn_in_trait, clippy::too_many_arguments)]
pub trait OwnedAccountTransactionListItemWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        name: CurrencyName,
        decimals: CurrencyDecimals,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        display_name: OrganizationDisplayName,
        picture: Option<OrganizationPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn insert_account_transaction(
        &self,
        uow: &mut Self::Uow,
        id: EventId,
        correlation_id: CorrelationId,
        account_id: AccountId,
        counterparty_account_id: Option<AccountId>,
        amount: CurrencyAmount,
        direction: OwnedAccountTransactionListItemDirection,
        kind: OwnedAccountTransactionListItemKind,
        status: OwnedAccountTransactionListItemStatus,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn record_transfer_requested(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        correlation_id: CorrelationId,
        from_account_id: AccountId,
        to_account_id: AccountId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn complete_transfer(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn fail_transfer(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        reason: TransferFailureReason,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn record_currency_issuance_issued(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        destination_account_id: AccountId,
        currency_id: CurrencyId,
        amount: CurrencyAmount,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn complete_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;

    async fn fail_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListItemWriterError>;
}
