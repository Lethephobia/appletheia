pub mod command_envelope;
pub mod command_outbox_fetcher;
pub mod command_outbox_fetcher_access;
pub mod command_outbox_fetcher_error;
pub mod command_outbox_id;
pub mod command_outbox_id_error;
pub mod command_outbox_publish_result;
pub mod command_outbox_publisher;
pub mod command_outbox_publisher_access;
pub mod command_outbox_publisher_error;
pub mod command_outbox_relay;
pub mod command_outbox_relay_config_access;
pub mod command_outbox_relay_error;
pub mod command_outbox_writer;
pub mod command_outbox_writer_access;
pub mod command_outbox_writer_error;

pub use command_envelope::CommandEnvelope;
pub use command_outbox_fetcher::CommandOutboxFetcher;
pub use command_outbox_fetcher_access::CommandOutboxFetcherAccess;
pub use command_outbox_fetcher_error::CommandOutboxFetcherError;
pub use command_outbox_id::CommandOutboxId;
pub use command_outbox_id_error::CommandOutboxIdError;
pub use command_outbox_publish_result::CommandOutboxPublishResult;
pub use command_outbox_publisher::CommandOutboxPublisher;
pub use command_outbox_publisher_access::CommandOutboxPublisherAccess;
pub use command_outbox_publisher_error::CommandOutboxPublisherError;
pub use command_outbox_relay::CommandOutboxRelay;
pub use command_outbox_relay_config_access::CommandOutboxRelayConfigAccess;
pub use command_outbox_relay_error::CommandOutboxRelayError;
pub use command_outbox_writer::CommandOutboxWriter;
pub use command_outbox_writer_access::CommandOutboxWriterAccess;
pub use command_outbox_writer_error::CommandOutboxWriterError;

pub type CommandOutboxAttemptCount = crate::event::event_outbox::EventOutboxAttemptCount;
pub type CommandOutboxAttemptCountError = crate::event::event_outbox::EventOutboxAttemptCountError;
pub type CommandOutboxBatchSize = crate::event::event_outbox::EventOutboxBatchSize;
pub type CommandOutboxDeadLetteredAt = crate::event::event_outbox::EventOutboxDeadLetteredAt;
pub type CommandOutboxDispatchError = crate::event::event_outbox::EventOutboxDispatchError;
pub type CommandOutboxError = crate::event::event_outbox::EventOutboxError;
pub type CommandOutboxLeaseDuration = crate::event::event_outbox::EventOutboxLeaseDuration;
pub type CommandOutboxLeaseExpiresAt = crate::event::event_outbox::EventOutboxLeaseExpiresAt;
pub type CommandOutboxLifecycle = crate::event::event_outbox::EventOutboxLifecycle;
pub type CommandOutboxMaxAttempts = crate::event::event_outbox::EventOutboxMaxAttempts;
pub type CommandOutboxNextAttemptAt = crate::event::event_outbox::EventOutboxNextAttemptAt;
pub type CommandOutboxPollBackoffMultiplier =
    crate::event::event_outbox::EventOutboxPollBackoffMultiplier;
pub type CommandOutboxPollBackoffMultiplierError =
    crate::event::event_outbox::EventOutboxPollBackoffMultiplierError;
pub type CommandOutboxPollInterval = crate::event::event_outbox::EventOutboxPollInterval;
pub type CommandOutboxPollJitterRatio = crate::event::event_outbox::EventOutboxPollJitterRatio;
pub type CommandOutboxPollJitterRatioError =
    crate::event::event_outbox::EventOutboxPollJitterRatioError;
pub type CommandOutboxPollingOptions = crate::event::event_outbox::EventOutboxPollingOptions;
pub type CommandOutboxPollingOptionsError =
    crate::event::event_outbox::EventOutboxPollingOptionsError;
pub type CommandOutboxPublishedAt = crate::event::event_outbox::EventOutboxPublishedAt;
pub type CommandOutboxRelayConfig = crate::event::event_outbox::EventOutboxRelayConfig;
pub type CommandOutboxRelayInstance = crate::event::event_outbox::EventOutboxRelayInstance;
pub type CommandOutboxRelayInstanceError =
    crate::event::event_outbox::EventOutboxRelayInstanceError;
pub type CommandOutboxRelayInstanceId = crate::event::event_outbox::EventOutboxRelayInstanceId;
pub type CommandOutboxRelayProcessId = crate::event::event_outbox::EventOutboxRelayProcessId;
pub type CommandOutboxRelayRunReport = crate::event::event_outbox::EventOutboxRelayRunReport;
pub type CommandOutboxRetryDelay = crate::event::event_outbox::EventOutboxRetryDelay;
pub type CommandOutboxRetryOptions = crate::event::event_outbox::EventOutboxRetryOptions;
pub type CommandOutboxState = crate::event::event_outbox::EventOutboxState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandOutbox {
    pub id: CommandOutboxId,
    pub sequence: i64,
    pub command: CommandEnvelope,
    pub state: CommandOutboxState,
    pub last_error: Option<CommandOutboxDispatchError>,
    pub lifecycle: CommandOutboxLifecycle,
}

impl CommandOutbox {
    pub fn ack(&mut self) -> Result<(), CommandOutboxError> {
        if matches!(self.lifecycle, CommandOutboxLifecycle::DeadLettered { .. }) {
            return Err(CommandOutboxError::AckOnDeadLettered(
                self.lifecycle.clone(),
            ));
        }

        let published_at = CommandOutboxPublishedAt::now();
        let attempt_count = self.state.attempt_count();

        self.state = CommandOutboxState::Published {
            published_at,
            attempt_count,
        };
        self.last_error = None;
        self.lifecycle = CommandOutboxLifecycle::Active;

        Ok(())
    }

    pub fn nack(
        &mut self,
        cause: &CommandOutboxDispatchError,
        retry_options: &CommandOutboxRetryOptions,
    ) -> Result<(), CommandOutboxError> {
        if matches!(self.lifecycle, CommandOutboxLifecycle::DeadLettered { .. }) {
            return Err(CommandOutboxError::NackOnDeadLettered(
                self.lifecycle.clone(),
            ));
        }

        self.last_error = Some(cause.clone());

        let current_attempt_count = self.state.attempt_count();
        let next_attempt_count = current_attempt_count
            .try_increment()
            .map_err(CommandOutboxError::AttemptCount)?;

        let maximum_attempts = retry_options.max_attempts.value().get() as i64;
        let has_exceeded_maximum_attempts = next_attempt_count.value() > maximum_attempts;

        if has_exceeded_maximum_attempts {
            let dead_lettered_at = CommandOutboxDeadLetteredAt::now();
            self.lifecycle = CommandOutboxLifecycle::DeadLettered { dead_lettered_at };
        } else {
            match cause {
                CommandOutboxDispatchError::Permanent { .. } => {
                    let dead_lettered_at = CommandOutboxDeadLetteredAt::now();
                    self.lifecycle = CommandOutboxLifecycle::DeadLettered { dead_lettered_at };
                }
                CommandOutboxDispatchError::Transient { .. } => {
                    let next_attempt_at =
                        CommandOutboxNextAttemptAt::now().next(retry_options.backoff);

                    self.state = CommandOutboxState::Pending {
                        attempt_count: next_attempt_count,
                        next_attempt_after: next_attempt_at,
                    };
                    self.lifecycle = CommandOutboxLifecycle::Active;
                }
            }
        }

        Ok(())
    }

    pub fn extend_lease(
        &mut self,
        owner: &CommandOutboxRelayInstance,
        lease_for: CommandOutboxLeaseDuration,
    ) -> Result<(), CommandOutboxError> {
        if matches!(self.lifecycle, CommandOutboxLifecycle::DeadLettered { .. }) {
            return Err(CommandOutboxError::ExtendLeaseOnDeadLettered(
                self.lifecycle.clone(),
            ));
        }

        let current_state = self.state.clone();
        let lease_expires_at = CommandOutboxLeaseExpiresAt::from_now(lease_for);

        match current_state {
            CommandOutboxState::Leased {
                attempt_count,
                next_attempt_after,
                ..
            } => {
                self.state = CommandOutboxState::Leased {
                    attempt_count,
                    next_attempt_after,
                    lease_owner: owner.clone(),
                    lease_until: lease_expires_at,
                };
                Ok(())
            }
            _ => Err(CommandOutboxError::ExtendLeaseOnNonLeased(current_state)),
        }
    }

    pub fn acquire_lease(
        &mut self,
        owner: &CommandOutboxRelayInstance,
        lease_for: CommandOutboxLeaseDuration,
    ) -> Result<(), CommandOutboxError> {
        if matches!(self.lifecycle, CommandOutboxLifecycle::DeadLettered { .. }) {
            return Err(CommandOutboxError::AcquireLeaseOnDeadLettered(
                self.lifecycle.clone(),
            ));
        }

        let current_state = self.state.clone();
        let lease_expires_at = CommandOutboxLeaseExpiresAt::from_now(lease_for);

        match current_state {
            CommandOutboxState::Pending {
                attempt_count,
                next_attempt_after,
            } => {
                self.state = CommandOutboxState::Leased {
                    attempt_count,
                    next_attempt_after,
                    lease_owner: owner.clone(),
                    lease_until: lease_expires_at,
                };
                Ok(())
            }
            _ => Err(CommandOutboxError::AcquireLeaseOnNonPending(current_state)),
        }
    }
}
