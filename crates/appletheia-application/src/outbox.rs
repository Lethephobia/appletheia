pub mod ordering_key;
pub mod ordering_key_error;
pub mod outbox_attempt_count;
pub mod outbox_attempt_count_error;
pub mod outbox_batch_size;
pub mod outbox_dead_lettered_at;
pub mod outbox_dispatch_error;
pub mod outbox_error;
pub mod outbox_fetcher;
pub mod outbox_fetcher_error;
pub mod outbox_fetcher_provider;
pub mod outbox_id;
pub mod outbox_id_error;
pub mod outbox_lease_duration;
pub mod outbox_lease_expires_at;
pub mod outbox_lifecycle;
pub mod outbox_max_attempts;
pub mod outbox_next_attempt_at;
pub mod outbox_publish_result;
pub mod outbox_published_at;
pub mod outbox_publisher;
pub mod outbox_publisher_access;
pub mod outbox_publisher_error;
pub mod outbox_relay;
pub mod outbox_relay_config;
pub mod outbox_relay_config_access;
pub mod outbox_relay_error;
pub mod outbox_relay_instance;
pub mod outbox_relay_instance_error;
pub mod outbox_relay_instance_id;
pub mod outbox_relay_process_id;
pub mod outbox_retry_delay;
pub mod outbox_retry_options;
pub mod outbox_state;
pub mod outbox_writer;
pub mod outbox_writer_error;
pub mod outbox_writer_provider;

pub use crate::event::AppEvent;
pub use ordering_key::OrderingKey;
pub use ordering_key_error::OrderingKeyError;
pub use outbox_attempt_count::OutboxAttemptCount;
pub use outbox_attempt_count_error::OutboxAttemptCountError;
pub use outbox_batch_size::OutboxBatchSize;
pub use outbox_dead_lettered_at::DeadLetteredAt;
pub use outbox_dispatch_error::OutboxDispatchError;
pub use outbox_error::OutboxError;
pub use outbox_fetcher::OutboxFetcher;
pub use outbox_fetcher_error::OutboxFetcherError;
pub use outbox_fetcher_provider::OutboxFetcherProvider;
pub use outbox_id::OutboxId;
pub use outbox_id_error::OutboxIdError;
pub use outbox_lease_duration::OutboxLeaseDuration;
pub use outbox_lease_expires_at::OutboxLeaseExpiresAt;
pub use outbox_lifecycle::OutboxLifecycle;
pub use outbox_max_attempts::OutboxMaxAttempts;
pub use outbox_next_attempt_at::OutboxNextAttemptAt;
pub use outbox_publish_result::OutboxPublishResult;
pub use outbox_published_at::OutboxPublishedAt;
pub use outbox_publisher::OutboxPublisher;
pub use outbox_publisher_access::OutboxPublisherAccess;
pub use outbox_publisher_error::OutboxPublisherError;
pub use outbox_relay::OutboxRelay;
pub use outbox_relay_config::OutboxRelayConfig;
pub use outbox_relay_config_access::OutboxRelayConfigAccess;
pub use outbox_relay_error::OutboxRelayError;
pub use outbox_relay_instance::OutboxRelayInstance;
pub use outbox_relay_instance_error::OutboxRelayInstanceError;
pub use outbox_relay_instance_id::OutboxRelayInstanceId;
pub use outbox_relay_process_id::OutboxRelayProcessId;
pub use outbox_retry_delay::OutboxRetryDelay;
pub use outbox_retry_options::OutboxRetryOptions;
pub use outbox_state::OutboxState;
pub use outbox_writer::OutboxWriter;
pub use outbox_writer_error::OutboxWriterError;
pub use outbox_writer_provider::OutboxWriterProvider;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outbox {
    pub id: OutboxId,
    pub event: AppEvent,
    pub state: OutboxState,
    pub last_error: Option<OutboxDispatchError>,
    pub lifecycle: OutboxLifecycle,
}

impl Outbox {
    pub fn ordering_key(&self) -> OrderingKey {
        OrderingKey::new(self.event.aggregate_type.clone(), self.event.aggregate_id)
    }

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
            let dead_lettered_at = DeadLetteredAt::now();
            self.lifecycle = OutboxLifecycle::DeadLettered { dead_lettered_at };
        } else {
            match cause {
                OutboxDispatchError::Permanent { .. } => {
                    let dead_lettered_at = DeadLetteredAt::now();
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
        owner: &OutboxRelayInstance,
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
        owner: &OutboxRelayInstance,
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
