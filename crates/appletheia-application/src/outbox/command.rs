pub mod command_envelope;
pub mod command_outbox_fetcher;
pub mod command_outbox_fetcher_access;
pub mod command_outbox_id;
pub mod command_outbox_id_error;
pub mod command_outbox_publish_result;
pub mod command_outbox_publisher;
pub mod command_outbox_publisher_access;
pub mod command_outbox_relay;
pub mod command_outbox_relay_error;
pub mod command_outbox_writer;
pub mod command_outbox_writer_access;

pub use command_envelope::CommandEnvelope;
pub use command_outbox_fetcher::CommandOutboxFetcher;
pub use command_outbox_fetcher_access::CommandOutboxFetcherAccess;
pub use command_outbox_id::CommandOutboxId;
pub use command_outbox_id_error::CommandOutboxIdError;
pub use command_outbox_publish_result::CommandOutboxPublishResult;
pub use command_outbox_publisher::CommandOutboxPublisher;
pub use command_outbox_publisher_access::CommandOutboxPublisherAccess;
pub use command_outbox_relay::CommandOutboxRelay;
pub use command_outbox_relay_error::CommandOutboxRelayError;
pub use command_outbox_writer::CommandOutboxWriter;
pub use command_outbox_writer_access::CommandOutboxWriterAccess;

use crate::outbox::{
    OutboxDeadLetteredAt, OutboxDispatchError, OutboxError, OutboxLeaseDuration,
    OutboxLeaseExpiresAt, OutboxLifecycle, OutboxNextAttemptAt, OutboxPublishedAt,
    OutboxRetryOptions, OutboxState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandOutbox {
    pub id: CommandOutboxId,
    pub sequence: i64,
    pub command: CommandEnvelope,
    pub state: OutboxState,
    pub last_error: Option<OutboxDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl CommandOutbox {
    pub fn ack(&mut self) -> Result<(), OutboxError> {
        if matches!(self.lifecycle, OutboxLifecycle::DeadLettered { .. }) {
            return Err(OutboxError::AckOnDeadLettered(self.lifecycle.clone()));
        }

        let published_at = OutboxPublishedAt::now();
        let attempt_count = self.state.attempt_count();

        self.state = OutboxState::Published {
            published_at,
            attempt_count,
        };
        self.last_error = None;
        self.lifecycle = OutboxLifecycle::Active;

        Ok(())
    }

    pub fn nack(
        &mut self,
        cause: &OutboxDispatchError,
        retry_options: &OutboxRetryOptions,
    ) -> Result<(), OutboxError> {
        if matches!(self.lifecycle, OutboxLifecycle::DeadLettered { .. }) {
            return Err(OutboxError::NackOnDeadLettered(self.lifecycle.clone()));
        }

        self.last_error = Some(cause.clone());

        let current_attempt_count = self.state.attempt_count();
        let next_attempt_count = current_attempt_count
            .try_increment()
            .map_err(OutboxError::AttemptCount)?;

        let maximum_attempts = retry_options.max_attempts.value().get() as i64;
        let has_exceeded_maximum_attempts = next_attempt_count.value() > maximum_attempts;

        if has_exceeded_maximum_attempts {
            let dead_lettered_at = OutboxDeadLetteredAt::now();
            self.lifecycle = OutboxLifecycle::DeadLettered { dead_lettered_at };
        } else {
            match cause {
                OutboxDispatchError::Permanent { .. } => {
                    let dead_lettered_at = OutboxDeadLetteredAt::now();
                    self.lifecycle = OutboxLifecycle::DeadLettered { dead_lettered_at };
                }
                OutboxDispatchError::Transient { .. } => {
                    let next_attempt_at = OutboxNextAttemptAt::now().next(retry_options.backoff);

                    self.state = OutboxState::Pending {
                        attempt_count: next_attempt_count,
                        next_attempt_after: next_attempt_at,
                    };
                    self.lifecycle = OutboxLifecycle::Active;
                }
            }
        }

        Ok(())
    }

    pub fn extend_lease(
        &mut self,
        owner: &super::OutboxRelayInstance,
        lease_for: OutboxLeaseDuration,
    ) -> Result<(), OutboxError> {
        if matches!(self.lifecycle, OutboxLifecycle::DeadLettered { .. }) {
            return Err(OutboxError::ExtendLeaseOnDeadLettered(
                self.lifecycle.clone(),
            ));
        }

        let current_state = self.state.clone();
        let lease_expires_at = OutboxLeaseExpiresAt::from_now(lease_for);

        match current_state {
            OutboxState::Leased {
                attempt_count,
                next_attempt_after,
                ..
            } => {
                self.state = OutboxState::Leased {
                    attempt_count,
                    next_attempt_after,
                    lease_owner: owner.clone(),
                    lease_until: lease_expires_at,
                };
                Ok(())
            }
            _ => Err(OutboxError::ExtendLeaseOnNonLeased(current_state)),
        }
    }

    pub fn acquire_lease(
        &mut self,
        owner: &super::OutboxRelayInstance,
        lease_for: OutboxLeaseDuration,
    ) -> Result<(), OutboxError> {
        if matches!(self.lifecycle, OutboxLifecycle::DeadLettered { .. }) {
            return Err(OutboxError::AcquireLeaseOnDeadLettered(
                self.lifecycle.clone(),
            ));
        }

        let current_state = self.state.clone();
        let lease_expires_at = OutboxLeaseExpiresAt::from_now(lease_for);

        match current_state {
            OutboxState::Pending {
                attempt_count,
                next_attempt_after,
            } => {
                self.state = OutboxState::Leased {
                    attempt_count,
                    next_attempt_after,
                    lease_owner: owner.clone(),
                    lease_until: lease_expires_at,
                };
                Ok(())
            }
            _ => Err(OutboxError::AcquireLeaseOnNonPending(current_state)),
        }
    }
}
