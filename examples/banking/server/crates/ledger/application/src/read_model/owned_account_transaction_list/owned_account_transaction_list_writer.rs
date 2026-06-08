use appletheia::application::unit_of_work::UnitOfWork;

use banking_iam_domain::{
    OrganizationDisplayName, OrganizationHandle, OrganizationId, OrganizationPictureRef,
    UserDisplayName, UserId, UserPictureRef, Username,
};
use banking_ledger_domain::currency::{CurrencyId, CurrencyName, CurrencySymbol};
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;
use banking_ledger_domain::transfer::{TransferFailureReason, TransferId};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

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
        event_context: ReadModelEventContext,
        upsert: OwnedAccountTransactionListCurrencyUpsert,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_currency_symbol(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        symbol: CurrencySymbol,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_currency_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
        name: CurrencyName,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn delete_currency(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyId,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn upsert_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OwnedAccountTransactionListOwnerUserUpsert,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_user_username(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        username: Username,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_user_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        display_name: UserDisplayName,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_user_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
        picture: Option<UserPictureRef>,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn delete_owner_user(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: UserId,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn upsert_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        upsert: OwnedAccountTransactionListOwnerOrganizationUpsert,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_organization_handle(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        handle: OrganizationHandle,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_organization_display_name(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        display_name: OrganizationDisplayName,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn update_owner_organization_picture(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
        picture: Option<OrganizationPictureRef>,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn delete_owner_organization(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: OrganizationId,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn insert_account_transaction(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        insert: OwnedAccountTransactionListItemInsert,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn record_transfer_requested(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        record: OwnedAccountTransactionListTransferRequestedRecord,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn complete_transfer(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: TransferId,
        transaction_id: OwnedAccountTransactionId,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn fail_transfer(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: TransferId,
        reason: TransferFailureReason,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn record_currency_issuance_issued(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        record: OwnedAccountTransactionListCurrencyIssuanceIssuedRecord,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn complete_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyIssuanceId,
        transaction_id: OwnedAccountTransactionId,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;

    async fn fail_currency_issuance(
        &self,
        uow: &mut Self::Uow,
        event_context: ReadModelEventContext,
        id: CurrencyIssuanceId,
    ) -> Result<(), OwnedAccountTransactionListWriterError>;
}
