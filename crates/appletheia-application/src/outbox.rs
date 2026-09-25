pub mod command;
pub mod command_failure;
pub mod event;
pub mod read_model_invalidation;

mod default_outbox_relay;
mod outbox_attempt_count;
mod outbox_attempt_count_error;
mod outbox_batch_size;
mod outbox_dead_lettered_at;
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
mod outbox_published_at;
mod outbox_relay;
mod outbox_relay_config;
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
mod processed_outbox_count;

pub use default_outbox_relay::*;
pub use outbox_attempt_count::*;
pub use outbox_attempt_count_error::*;
pub use outbox_batch_size::*;
pub use outbox_dead_lettered_at::*;
pub use outbox_error::*;
pub use outbox_fetcher::*;
pub use outbox_fetcher_error::*;
pub use outbox_lease_duration::*;
pub use outbox_lease_expires_at::*;
pub use outbox_lifecycle::*;
pub use outbox_max_attempts::*;
pub use outbox_next_attempt_at::*;
pub use outbox_poll_backoff_multiplier::*;
pub use outbox_poll_backoff_multiplier_error::*;
pub use outbox_poll_interval::*;
pub use outbox_poll_jitter_ratio::*;
pub use outbox_poll_jitter_ratio_error::*;
pub use outbox_polling_options::*;
pub use outbox_polling_options_error::*;
pub use outbox_published_at::*;
pub use outbox_relay::*;
pub use outbox_relay_config::*;
pub use outbox_relay_error::*;
pub use outbox_relay_instance::*;
pub use outbox_relay_instance_error::*;
pub use outbox_relay_instance_id::*;
pub use outbox_relay_process_id::*;
pub use outbox_relay_run_report::*;
pub use outbox_retry_delay::*;
pub use outbox_retry_options::*;
pub use outbox_state::*;
pub use outbox_writer::*;
pub use outbox_writer_error::*;
pub use processed_outbox_count::*;

use crate::messaging::{PublishDispatchError, PublishableMessage};

pub trait Outbox {
    type Id: Copy + Eq + 'static;
    type Message: PublishableMessage;

    fn id(&self) -> Self::Id;

    fn message(&self) -> &Self::Message;

    fn state(&self) -> &OutboxState;

    fn state_mut(&mut self) -> &mut OutboxState;

    fn last_error(&self) -> &Option<PublishDispatchError>;

    fn last_error_mut(&mut self) -> &mut Option<PublishDispatchError>;

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

    fn redrive(&mut self) -> Result<(), OutboxError> {
        match self.lifecycle() {
            OutboxLifecycle::DeadLettered { .. } => {}
            other => return Err(OutboxError::RedriveOnNonDeadLettered(other.clone())),
        }

        *self.state_mut() = OutboxState::Pending {
            attempt_count: OutboxAttemptCount::default(),
            next_attempt_after: OutboxNextAttemptAt::now(),
        };
        *self.last_error_mut() = None;
        *self.lifecycle_mut() = OutboxLifecycle::Active;

        Ok(())
    }

    fn nack(
        &mut self,
        cause: &PublishDispatchError,
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
                PublishDispatchError::Permanent { .. } => {
                    let dead_lettered_at = OutboxDeadLetteredAt::now();
                    *self.lifecycle_mut() = OutboxLifecycle::DeadLettered { dead_lettered_at };
                }
                PublishDispatchError::Transient { .. } => {
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
