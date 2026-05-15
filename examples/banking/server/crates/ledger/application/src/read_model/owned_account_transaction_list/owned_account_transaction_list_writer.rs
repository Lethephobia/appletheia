use appletheia::application::event::EventSequence;
use appletheia::application::unit_of_work::UnitOfWork;
use appletheia::domain::EventId;
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_domain::currency::{CurrencyId, CurrencyName, CurrencySymbol};
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use banking_ledger_domain::transfer::{TransferFailureReason, TransferId};

use super::{
    OwnedAccountTransactionId, OwnedAccountTransactionListCurrencyIssuanceIssuedRecord,
    OwnedAccountTransactionListCurrencyUpsert, OwnedAccountTransactionListItemInsert,
    OwnedAccountTransactionListOwnerOrganizationUpsert, OwnedAccountTransactionListOwnerUserUpsert,
    OwnedAccountTransactionListTransferRequestedRecord, OwnedAccountTransactionListWriterError,
};

#[allow(async_fn_in_trait)]
pub trait OwnedAccountTransactionListWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn upsert_currency(
        &self,
        uow: &mut Self::Uow,
        upsert: OwnedAccountTransactionListCurrencyUpsert,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        symbol: CurrencySymbol,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        name: CurrencyName,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyId,
        event_id: EventId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        upsert: OwnedAccountTransactionListOwnerUserUpsert,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        username: Username,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        display_name: UserDisplayName,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        picture: Option<UserPictureRef>,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        id: UserId,
        event_id: EventId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        upsert: OwnedAccountTransactionListOwnerOrganizationUpsert,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        handle: OrganizationHandle,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        id: OrganizationId,
        event_id: EventId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn insert_account_transaction(
        &self,
        uow: &mut Self::Uow,
        insert: OwnedAccountTransactionListItemInsert,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn record_transfer_requested(
        &self,
        uow: &mut Self::Uow,
        record: OwnedAccountTransactionListTransferRequestedRecord,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn complete_transfer(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        transaction_id: OwnedAccountTransactionId,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn fail_transfer(
        &self,
        uow: &mut Self::Uow,
        id: TransferId,
        reason: TransferFailureReason,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn record_currency_issuance_issued(
        &self,
        uow: &mut Self::Uow,
        record: OwnedAccountTransactionListCurrencyIssuanceIssuedRecord,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn complete_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        transaction_id: OwnedAccountTransactionId,
        event_id: EventId,
        event_sequence: EventSequence,
        occurred_at: EventOccurredAt,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn fail_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        id: CurrencyIssuanceId,
        event_id: EventId,
        event_sequence: EventSequence,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;
}
