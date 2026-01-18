use std::time::Duration as StdDuration;

use chrono::Duration;
use tokio::time::sleep;

use crate::outbox::{
    OutboxLeaseDuration, OutboxRelayConfigAccess, OutboxRelayInstance, OutboxRelayRunReport,
    OutboxRetryOptions, OutboxState,
};
use crate::unit_of_work::{UnitOfWork, UnitOfWorkError};

use super::{
    Outbox, OutboxFetcher, OutboxPublishResult, OutboxPublisher, OutboxRelayError, OutboxWriter,
};

#[allow(async_fn_in_trait)]
pub trait OutboxRelay: OutboxRelayConfigAccess {
    type Uow: UnitOfWork;
    type Outbox: Outbox;

    type Fetcher: OutboxFetcher<Uow = Self::Uow, Outbox = Self::Outbox>;
    type Writer: OutboxWriter<Uow = Self::Uow, Outbox = Self::Outbox>;
    type Publisher: OutboxPublisher<Outbox = Self::Outbox>;

    fn is_stop_requested(&self) -> bool;

    fn request_graceful_stop(&mut self);

    fn outbox_fetcher(&self) -> &Self::Fetcher;

    fn outbox_writer(&self) -> &Self::Writer;

    fn outbox_publisher(&self) -> &Self::Publisher;

    async fn run_forever(&self, uow: &mut Self::Uow) -> Result<(), OutboxRelayError> {
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
    ) -> Result<OutboxRelayRunReport, OutboxRelayError> {
        if uow.is_in_transaction() {
            return Err(UnitOfWorkError::AlreadyInTransaction.into());
        }

        let relay_config = self.config();
        let relay_instance: &OutboxRelayInstance = &relay_config.instance;
        let lease_duration: OutboxLeaseDuration = relay_config.lease_duration;
        let batch_size = relay_config.batch_size;
        let retry_options: OutboxRetryOptions = relay_config.retry_options;

        let outbox_fetcher = self.outbox_fetcher();
        let outbox_writer_for_lease = self.outbox_writer();

        uow.begin().await?;

        let operation_result: Result<Vec<Self::Outbox>, OutboxRelayError> = async {
            let mut outboxes = outbox_fetcher.fetch(uow, batch_size).await?;

            if outboxes.is_empty() {
                return Ok(outboxes);
            }

            for outbox in &mut outboxes {
                match outbox.state() {
                    OutboxState::Pending { .. } => {
                        outbox.acquire_lease(relay_instance, lease_duration)?;
                    }
                    other => return Err(OutboxRelayError::NonPendingOutboxState(other.clone())),
                }
            }

            outbox_writer_for_lease.write_outbox(uow, &outboxes).await?;

            Ok(outboxes)
        }
        .await;

        let mut outboxes: Vec<Self::Outbox> = match operation_result {
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
                OutboxPublishResult::Success { input_index, .. } => {
                    outboxes[input_index].ack()?;
                }
                OutboxPublishResult::Failed {
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

        let operation_result: Result<(), OutboxRelayError> = async {
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
