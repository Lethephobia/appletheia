use appletheia::application::unit_of_work::UnitOfWork;

use appletheia::application::read_model::MaterializationEventContext;
use banking_ledger_domain::core::OnchainTransactionId;
use banking_ledger_domain::transfer::{TransferFailureReason, TransferId};

use super::{
    AccountTransactionFragment, AccountTransactionFragmentInsert,
    AccountTransactionFragmentWriterError, AccountTransactionId, AccountTransactionStatus,
    AccountTransactionTransferRequestedRecord,
};

#[allow(async_fn_in_trait)]
pub trait AccountTransactionFragmentWriter: Send + Sync {
    type Uow: UnitOfWork;

    async fn insert_account_transaction(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        insert: AccountTransactionFragmentInsert,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError>;

    async fn update_account_transaction_status(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountTransactionId,
        status: AccountTransactionStatus,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError>;

    async fn record_onchain_transaction(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: AccountTransactionId,
        transaction_id: OnchainTransactionId,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError>;

    async fn record_transfer_requested(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        record: AccountTransactionTransferRequestedRecord,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError>;

    async fn complete_transfer(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: TransferId,
        transaction_id: AccountTransactionId,
    ) -> Result<Vec<AccountTransactionFragment>, AccountTransactionFragmentWriterError>;

    async fn fail_transfer(
        &self,
        uow: &mut Self::Uow,
        event_context: MaterializationEventContext,
        id: TransferId,
        reason: TransferFailureReason,
    ) -> Result<Option<AccountTransactionFragment>, AccountTransactionFragmentWriterError>;
}
