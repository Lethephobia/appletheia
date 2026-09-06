use crate::request_context::MessageId;
use crate::unit_of_work::UnitOfWork;

use super::{
    CommandAttemptCount, CommandEnvelope, CommandExecutionFailureMarkResult,
    CommandExecutionLeaseAcquisitionResult, CommandExecutionLeaseDuration,
    CommandExecutionLeaseReleaseResult, CommandExecutionStoreError,
};

/// Persists transport-independent command execution attempts and terminality.
#[allow(async_fn_in_trait)]
pub trait CommandExecutionStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn acquire_lease(
        &self,
        uow: &mut Self::Uow,
        command: &CommandEnvelope,
        lease_duration: CommandExecutionLeaseDuration,
    ) -> Result<CommandExecutionLeaseAcquisitionResult, CommandExecutionStoreError>;

    async fn mark_succeeded(
        &self,
        uow: &mut Self::Uow,
        command_message_id: MessageId,
    ) -> Result<(), CommandExecutionStoreError>;

    async fn release_lease(
        &self,
        uow: &mut Self::Uow,
        command_message_id: MessageId,
        attempt_count: CommandAttemptCount,
    ) -> Result<CommandExecutionLeaseReleaseResult, CommandExecutionStoreError>;

    async fn mark_failed(
        &self,
        uow: &mut Self::Uow,
        command_message_id: MessageId,
        attempt_count: CommandAttemptCount,
    ) -> Result<CommandExecutionFailureMarkResult, CommandExecutionStoreError>;
}
