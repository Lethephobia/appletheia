pub mod command;
pub mod event;

mod ordering_key;
mod outbox_attempt_count;
mod outbox_attempt_count_error;
mod outbox_batch_size;
mod outbox_dead_lettered_at;
mod outbox_dispatch_error;
mod outbox_error;
mod outbox_fetcher;
mod outbox_fetcher_error;
mod outbox_lease_duration;
mod outbox_lease_expires_at;
mod outbox_lifecycle;
mod outbox_max_attempts;
mod outbox_next_attempt_at;
mod outbox_poll_backoff_multiplier;
mod outbox_poll_backoff_multiplier_error;
mod outbox_poll_interval;
mod outbox_poll_jitter_ratio;
mod outbox_poll_jitter_ratio_error;
mod outbox_polling_options;
mod outbox_polling_options_error;
mod outbox_publish_result;
mod outbox_published_at;
mod outbox_publisher;
mod outbox_publisher_error;
mod outbox_relay;
mod outbox_relay_config;
mod outbox_relay_config_access;
mod outbox_relay_error;
mod outbox_relay_instance;
mod outbox_relay_instance_error;
mod outbox_relay_instance_id;
mod outbox_relay_process_id;
mod outbox_relay_run_report;
mod outbox_retry_delay;
mod outbox_retry_options;
mod outbox_state;
mod outbox_writer;
mod outbox_writer_error;

pub use ordering_key::{OrderingKey, OrderingKeyError};
pub use outbox_attempt_count::OutboxAttemptCount;
pub use outbox_attempt_count_error::OutboxAttemptCountError;
pub use outbox_batch_size::OutboxBatchSize;
pub use outbox_dead_lettered_at::OutboxDeadLetteredAt;
pub use outbox_dispatch_error::OutboxDispatchError;
pub use outbox_error::OutboxError;
pub use outbox_fetcher::OutboxFetcher;
pub use outbox_fetcher_error::OutboxFetcherError;
pub use outbox_lease_duration::OutboxLeaseDuration;
pub use outbox_lease_expires_at::OutboxLeaseExpiresAt;
pub use outbox_lifecycle::OutboxLifecycle;
pub use outbox_max_attempts::OutboxMaxAttempts;
pub use outbox_next_attempt_at::OutboxNextAttemptAt;
pub use outbox_poll_backoff_multiplier::OutboxPollBackoffMultiplier;
pub use outbox_poll_backoff_multiplier_error::OutboxPollBackoffMultiplierError;
pub use outbox_poll_interval::OutboxPollInterval;
pub use outbox_poll_jitter_ratio::OutboxPollJitterRatio;
pub use outbox_poll_jitter_ratio_error::OutboxPollJitterRatioError;
pub use outbox_polling_options::OutboxPollingOptions;
pub use outbox_polling_options_error::OutboxPollingOptionsError;
pub use outbox_publish_result::OutboxPublishResult;
pub use outbox_published_at::OutboxPublishedAt;
pub use outbox_publisher::OutboxPublisher;
pub use outbox_publisher_error::OutboxPublisherError;
pub use outbox_relay::OutboxRelay;
pub use outbox_relay_config::OutboxRelayConfig;
pub use outbox_relay_config_access::OutboxRelayConfigAccess;
pub use outbox_relay_error::OutboxRelayError;
pub use outbox_relay_instance::OutboxRelayInstance;
pub use outbox_relay_instance_error::OutboxRelayInstanceError;
pub use outbox_relay_instance_id::OutboxRelayInstanceId;
pub use outbox_relay_process_id::OutboxRelayProcessId;
pub use outbox_relay_run_report::OutboxRelayRunReport;
pub use outbox_retry_delay::OutboxRetryDelay;
pub use outbox_retry_options::OutboxRetryOptions;
pub use outbox_state::OutboxState;
pub use outbox_writer::OutboxWriter;
pub use outbox_writer_error::OutboxWriterError;

pub trait Outbox {
    type Id: Copy + Eq + 'static;

    fn id(&self) -> Self::Id;

    fn ordering_key(&self) -> &OrderingKey;

    fn state(&self) -> &OutboxState;

    fn state_mut(&mut self) -> &mut OutboxState;

    fn last_error(&self) -> &Option<OutboxDispatchError>;

    fn last_error_mut(&mut self) -> &mut Option<OutboxDispatchError>;

    fn lifecycle(&self) -> &OutboxLifecycle;

    fn lifecycle_mut(&mut self) -> &mut OutboxLifecycle;

    fn ack(&mut self) -> Result<(), OutboxError> {
        if matches!(self.lifecycle(), OutboxLifecycle::DeadLettered { .. }) {
            return Err(OutboxError::AckOnDeadLettered(self.lifecycle().clone()));
        }

        let published_at = OutboxPublishedAt::now();
        let attempt_count = self.state().attempt_count();

        *self.state_mut() = OutboxState::Published {
            published_at,
            attempt_count,
        };
        *self.last_error_mut() = None;
        *self.lifecycle_mut() = OutboxLifecycle::Active;

        Ok(())
    }

    fn nack(
        &mut self,
        cause: &OutboxDispatchError,
        retry_options: &OutboxRetryOptions,
    ) -> Result<(), OutboxError> {
        if matches!(self.lifecycle(), OutboxLifecycle::DeadLettered { .. }) {
            return Err(OutboxError::NackOnDeadLettered(self.lifecycle().clone()));
        }

        *self.last_error_mut() = Some(cause.clone());

        let current_attempt_count = self.state().attempt_count();
        let next_attempt_count = current_attempt_count
            .try_increment()
            .map_err(OutboxError::AttemptCount)?;

        let maximum_attempts = retry_options.max_attempts.value().get() as i64;
        let has_exceeded_maximum_attempts = next_attempt_count.value() > maximum_attempts;

        if has_exceeded_maximum_attempts {
            let dead_lettered_at = OutboxDeadLetteredAt::now();
            *self.lifecycle_mut() = OutboxLifecycle::DeadLettered { dead_lettered_at };
        } else {
            match cause {
                OutboxDispatchError::Permanent { .. } => {
                    let dead_lettered_at = OutboxDeadLetteredAt::now();
                    *self.lifecycle_mut() = OutboxLifecycle::DeadLettered { dead_lettered_at };
                }
                OutboxDispatchError::Transient { .. } => {
                    let next_attempt_at = OutboxNextAttemptAt::now().next(retry_options.backoff);

                    *self.state_mut() = OutboxState::Pending {
                        attempt_count: next_attempt_count,
                        next_attempt_after: next_attempt_at,
                    };
                    *self.lifecycle_mut() = OutboxLifecycle::Active;
                }
            }
        }

        Ok(())
    }

    fn extend_lease(
        &mut self,
        owner: &OutboxRelayInstance,
        lease_for: OutboxLeaseDuration,
    ) -> Result<(), OutboxError> {
        if matches!(self.lifecycle(), OutboxLifecycle::DeadLettered { .. }) {
            return Err(OutboxError::ExtendLeaseOnDeadLettered(
                self.lifecycle().clone(),
            ));
        }

        let current_state = self.state().clone();
        let lease_expires_at = OutboxLeaseExpiresAt::from_now(lease_for);

        match current_state {
            OutboxState::Leased {
                attempt_count,
                next_attempt_after,
                ..
            } => {
                *self.state_mut() = OutboxState::Leased {
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

    fn acquire_lease(
        &mut self,
        owner: &OutboxRelayInstance,
        lease_for: OutboxLeaseDuration,
    ) -> Result<(), OutboxError> {
        if matches!(self.lifecycle(), OutboxLifecycle::DeadLettered { .. }) {
            return Err(OutboxError::AcquireLeaseOnDeadLettered(
                self.lifecycle().clone(),
            ));
        }

        let current_state = self.state().clone();
        let lease_expires_at = OutboxLeaseExpiresAt::from_now(lease_for);

        match current_state {
            OutboxState::Pending {
                attempt_count,
                next_attempt_after,
            } => {
                *self.state_mut() = OutboxState::Leased {
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
