use std::time::Duration as StdDuration;

use chrono::Duration;
use tokio::time::sleep;

use super::{
    CommandOutbox, CommandOutboxFetcher, CommandOutboxFetcherAccess, CommandOutboxPublishResult,
    CommandOutboxPublisher, CommandOutboxPublisherAccess, CommandOutboxRelayError,
    CommandOutboxWriter, CommandOutboxWriterAccess,
};
use crate::outbox::{OutboxRelayConfigAccess, OutboxRelayRunReport, OutboxState};
use crate::unit_of_work::{UnitOfWork, UnitOfWorkError};

#[allow(async_fn_in_trait)]
pub trait CommandOutboxRelay:
    OutboxRelayConfigAccess
    + CommandOutboxPublisherAccess
    + CommandOutboxFetcherAccess
    + CommandOutboxWriterAccess
where
    <Self as CommandOutboxFetcherAccess>::Fetcher: CommandOutboxFetcher<Uow = Self::Uow>,
    <Self as CommandOutboxWriterAccess>::Writer: CommandOutboxWriter<Uow = Self::Uow>,
{
    type Uow: UnitOfWork;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    async fn run_forever(&self, uow: &mut Self::Uow) -> Result<(), CommandOutboxRelayError> {
        let relay_config = self.config();
        let polling_options = &relay_config.polling_options;
        let mut poll_interval = polling_options.base;

        while !self.is_stop_requested() {
            let run_report = self.run_once(uow).await?;

            match run_report {
                OutboxRelayRunReport::Progress { .. } => {
                    poll_interval = polling_options.base;
                }
                OutboxRelayRunReport::Idle | OutboxRelayRunReport::Throttled => {
                    let duration: Duration = poll_interval.into();
                    let sleep_duration = duration
                        .to_std()
                        .unwrap_or_else(|_| StdDuration::from_secs(0));

                    if sleep_duration > StdDuration::from_secs(0) {
                        sleep(sleep_duration).await;
                    }

                    poll_interval = poll_interval.next(
                        polling_options.multiplier,
                        polling_options.jitter,
                        polling_options.max,
                    );
                }
            }
        }

        Ok(())
    }

    async fn run_once(
        &self,
        uow: &mut Self::Uow,
    ) -> Result<OutboxRelayRunReport, CommandOutboxRelayError> {
        if uow.is_in_transaction() {
            return Err(UnitOfWorkError::AlreadyInTransaction.into());
        }

        let relay_config = self.config();
        let relay_instance = &relay_config.instance;
        let lease_duration = relay_config.lease_duration;
        let batch_size = relay_config.batch_size;
        let retry_options = relay_config.retry_options;

        let outbox_fetcher = self.outbox_fetcher();
        let outbox_writer_for_lease = self.outbox_writer();

        uow.begin().await?;

        let operation_result: Result<Vec<CommandOutbox>, CommandOutboxRelayError> = async {
            let mut outboxes = outbox_fetcher.fetch(uow, batch_size).await?;

            if outboxes.is_empty() {
                return Ok(outboxes);
            }

            for outbox in &mut outboxes {
                match &outbox.state {
                    OutboxState::Pending { .. } => {
                        outbox.acquire_lease(relay_instance, lease_duration)?;
                    }
                    other => {
                        return Err(CommandOutboxRelayError::NonPendingOutboxState(
                            other.clone(),
                        ));
                    }
                }
            }

            outbox_writer_for_lease.write_outbox(uow, &outboxes).await?;

            Ok(outboxes)
        }
        .await;

        let mut outboxes: Vec<CommandOutbox> = match operation_result {
            Ok(value) => {
                uow.commit().await?;
                value
            }
            Err(error) => return Err(uow.rollback_with_operation_error(error).await?),
        };

        if outboxes.is_empty() {
            return Ok(OutboxRelayRunReport::Idle);
        }

        let outbox_publisher = self.outbox_publisher();
        let publish_results = outbox_publisher.publish_outbox(&outboxes).await?;

        for publish_result in publish_results {
            match publish_result {
                CommandOutboxPublishResult::Success { input_index, .. } => {
                    outboxes[input_index].ack()?;
                }
                CommandOutboxPublishResult::Failed {
                    input_index,
                    ref cause,
                    ..
                } => {
                    outboxes[input_index].nack(cause, &retry_options)?;
                }
            }
        }

        let proceeded = outboxes.len().min(u32::MAX as usize) as u32;

        let outbox_writer_for_update = self.outbox_writer();

        uow.begin().await?;

        let operation_result: Result<(), CommandOutboxRelayError> = async {
            outbox_writer_for_update
                .write_outbox(uow, &outboxes)
                .await?;
            Ok(())
        }
        .await;

        match operation_result {
            Ok(()) => {
                uow.commit().await?;
            }
            Err(error) => return Err(uow.rollback_with_operation_error(error).await?),
        }

        Ok(OutboxRelayRunReport::Progress { proceeded })
    }
}
