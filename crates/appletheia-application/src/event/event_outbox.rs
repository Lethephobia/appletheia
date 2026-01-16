pub mod event_outbox_attempt_count;
pub mod event_outbox_attempt_count_error;
pub mod event_outbox_batch_size;
pub mod event_outbox_dead_lettered_at;
pub mod event_outbox_dispatch_error;
pub mod event_outbox_error;
pub mod event_outbox_fetcher;
pub mod event_outbox_fetcher_access;
pub mod event_outbox_fetcher_error;
pub mod event_outbox_id;
pub mod event_outbox_id_error;
pub mod event_outbox_lease_duration;
pub mod event_outbox_lease_expires_at;
pub mod event_outbox_lifecycle;
pub mod event_outbox_max_attempts;
pub mod event_outbox_next_attempt_at;
pub mod event_outbox_ordering_key;
pub mod event_outbox_ordering_key_error;
pub mod event_outbox_poll_backoff_multiplier;
pub mod event_outbox_poll_backoff_multiplier_error;
pub mod event_outbox_poll_interval;
pub mod event_outbox_poll_jitter_ratio;
pub mod event_outbox_poll_jitter_ratio_error;
pub mod event_outbox_polling_options;
pub mod event_outbox_polling_options_error;
pub mod event_outbox_publish_result;
pub mod event_outbox_published_at;
pub mod event_outbox_publisher;
pub mod event_outbox_publisher_access;
pub mod event_outbox_publisher_error;
pub mod event_outbox_relay;
pub mod event_outbox_relay_config;
pub mod event_outbox_relay_config_access;
pub mod event_outbox_relay_error;
pub mod event_outbox_relay_instance;
pub mod event_outbox_relay_instance_error;
pub mod event_outbox_relay_instance_id;
pub mod event_outbox_relay_process_id;
pub mod event_outbox_relay_run_report;
pub mod event_outbox_retry_delay;
pub mod event_outbox_retry_options;
pub mod event_outbox_state;
pub mod event_outbox_writer;
pub mod event_outbox_writer_access;
pub mod event_outbox_writer_error;

pub use crate::event::AppEvent;
pub use event_outbox_attempt_count::EventOutboxAttemptCount;
pub use event_outbox_attempt_count_error::EventOutboxAttemptCountError;
pub use event_outbox_batch_size::EventOutboxBatchSize;
pub use event_outbox_dead_lettered_at::EventOutboxDeadLetteredAt;
pub use event_outbox_dispatch_error::EventOutboxDispatchError;
pub use event_outbox_error::EventOutboxError;
pub use event_outbox_fetcher::EventOutboxFetcher;
pub use event_outbox_fetcher_access::EventOutboxFetcherAccess;
pub use event_outbox_fetcher_error::EventOutboxFetcherError;
pub use event_outbox_id::EventOutboxId;
pub use event_outbox_id_error::EventOutboxIdError;
pub use event_outbox_lease_duration::EventOutboxLeaseDuration;
pub use event_outbox_lease_expires_at::EventOutboxLeaseExpiresAt;
pub use event_outbox_lifecycle::EventOutboxLifecycle;
pub use event_outbox_max_attempts::EventOutboxMaxAttempts;
pub use event_outbox_next_attempt_at::EventOutboxNextAttemptAt;
pub use event_outbox_ordering_key::OrderingKey;
pub use event_outbox_ordering_key_error::OrderingKeyError;
pub use event_outbox_poll_backoff_multiplier::EventOutboxPollBackoffMultiplier;
pub use event_outbox_poll_backoff_multiplier_error::EventOutboxPollBackoffMultiplierError;
pub use event_outbox_poll_interval::EventOutboxPollInterval;
pub use event_outbox_poll_jitter_ratio::EventOutboxPollJitterRatio;
pub use event_outbox_poll_jitter_ratio_error::EventOutboxPollJitterRatioError;
pub use event_outbox_polling_options::EventOutboxPollingOptions;
pub use event_outbox_polling_options_error::EventOutboxPollingOptionsError;
pub use event_outbox_publish_result::EventOutboxPublishResult;
pub use event_outbox_published_at::EventOutboxPublishedAt;
pub use event_outbox_publisher::EventOutboxPublisher;
pub use event_outbox_publisher_access::EventOutboxPublisherAccess;
pub use event_outbox_publisher_error::EventOutboxPublisherError;
pub use event_outbox_relay::EventOutboxRelay;
pub use event_outbox_relay_config::EventOutboxRelayConfig;
pub use event_outbox_relay_config_access::EventOutboxRelayConfigAccess;
pub use event_outbox_relay_error::EventOutboxRelayError;
pub use event_outbox_relay_instance::EventOutboxRelayInstance;
pub use event_outbox_relay_instance_error::EventOutboxRelayInstanceError;
pub use event_outbox_relay_instance_id::EventOutboxRelayInstanceId;
pub use event_outbox_relay_process_id::EventOutboxRelayProcessId;
pub use event_outbox_relay_run_report::EventOutboxRelayRunReport;
pub use event_outbox_retry_delay::EventOutboxRetryDelay;
pub use event_outbox_retry_options::EventOutboxRetryOptions;
pub use event_outbox_state::EventOutboxState;
pub use event_outbox_writer::EventOutboxWriter;
pub use event_outbox_writer_access::EventOutboxWriterAccess;
pub use event_outbox_writer_error::EventOutboxWriterError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventOutbox {
    pub id: EventOutboxId,
    pub event: AppEvent,
    pub state: EventOutboxState,
    pub last_error: Option<EventOutboxDispatchError>,
    pub lifecycle: EventOutboxLifecycle,
}

impl EventOutbox {
    pub fn ordering_key(&self) -> OrderingKey {
        OrderingKey::new(self.event.aggregate_type.clone(), self.event.aggregate_id)
    }

    pub fn ack(&mut self) -> Result<(), EventOutboxError> {
        if matches!(self.lifecycle, EventOutboxLifecycle::DeadLettered { .. }) {
            return Err(EventOutboxError::AckOnDeadLettered(self.lifecycle.clone()));
        }

        let published_at = EventOutboxPublishedAt::now();
        let attempt_count = self.state.attempt_count();

        self.state = EventOutboxState::Published {
            published_at,
            attempt_count,
        };
        self.last_error = None;
        self.lifecycle = EventOutboxLifecycle::Active;

        Ok(())
    }

    pub fn nack(
        &mut self,
        cause: &EventOutboxDispatchError,
        retry_options: &EventOutboxRetryOptions,
    ) -> Result<(), EventOutboxError> {
        if matches!(self.lifecycle, EventOutboxLifecycle::DeadLettered { .. }) {
            return Err(EventOutboxError::NackOnDeadLettered(self.lifecycle.clone()));
        }

        self.last_error = Some(cause.clone());

        let current_attempt_count = self.state.attempt_count();
        let next_attempt_count = current_attempt_count
            .try_increment()
            .map_err(EventOutboxError::AttemptCount)?;

        let maximum_attempts = retry_options.max_attempts.value().get() as i64;
        let has_exceeded_maximum_attempts = next_attempt_count.value() > maximum_attempts;

        if has_exceeded_maximum_attempts {
            let dead_lettered_at = EventOutboxDeadLetteredAt::now();
            self.lifecycle = EventOutboxLifecycle::DeadLettered { dead_lettered_at };
        } else {
            match cause {
                EventOutboxDispatchError::Permanent { .. } => {
                    let dead_lettered_at = EventOutboxDeadLetteredAt::now();
                    self.lifecycle = EventOutboxLifecycle::DeadLettered { dead_lettered_at };
                }
                EventOutboxDispatchError::Transient { .. } => {
                    let next_attempt_at =
                        EventOutboxNextAttemptAt::now().next(retry_options.backoff);

                    self.state = EventOutboxState::Pending {
                        attempt_count: next_attempt_count,
                        next_attempt_after: next_attempt_at,
                    };
                    self.lifecycle = EventOutboxLifecycle::Active;
                }
            }
        }

        Ok(())
    }

    pub fn extend_lease(
        &mut self,
        owner: &EventOutboxRelayInstance,
        lease_for: EventOutboxLeaseDuration,
    ) -> Result<(), EventOutboxError> {
        if matches!(self.lifecycle, EventOutboxLifecycle::DeadLettered { .. }) {
            return Err(EventOutboxError::ExtendLeaseOnDeadLettered(
                self.lifecycle.clone(),
            ));
        }

        let current_state = self.state.clone();
        let lease_expires_at = EventOutboxLeaseExpiresAt::from_now(lease_for);

        match current_state {
            EventOutboxState::Leased {
                attempt_count,
                next_attempt_after,
                ..
            } => {
                self.state = EventOutboxState::Leased {
                    attempt_count,
                    next_attempt_after,
                    lease_owner: owner.clone(),
                    lease_until: lease_expires_at,
                };
                Ok(())
            }
            _ => Err(EventOutboxError::ExtendLeaseOnNonLeased(current_state)),
        }
    }

    pub fn acquire_lease(
        &mut self,
        owner: &EventOutboxRelayInstance,
        lease_for: EventOutboxLeaseDuration,
    ) -> Result<(), EventOutboxError> {
        if matches!(self.lifecycle, EventOutboxLifecycle::DeadLettered { .. }) {
            return Err(EventOutboxError::AcquireLeaseOnDeadLettered(
                self.lifecycle.clone(),
            ));
        }

        let current_state = self.state.clone();
        let lease_expires_at = EventOutboxLeaseExpiresAt::from_now(lease_for);

        match current_state {
            EventOutboxState::Pending {
                attempt_count,
                next_attempt_after,
            } => {
                self.state = EventOutboxState::Leased {
                    attempt_count,
                    next_attempt_after,
                    lease_owner: owner.clone(),
                    lease_until: lease_expires_at,
                };
                Ok(())
            }
            _ => Err(EventOutboxError::AcquireLeaseOnNonPending(current_state)),
        }
    }
}
