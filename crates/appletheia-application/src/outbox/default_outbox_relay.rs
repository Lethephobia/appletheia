use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::time::Duration as StdDuration;

use chrono::Duration;
use tokio::time::sleep;

use crate::unit_of_work::{UnitOfWork, UnitOfWorkError};

use super::{
    Outbox, OutboxFetcher, OutboxPublishResult, OutboxPublisher, OutboxRelay, OutboxRelayConfig,
    OutboxRelayError, OutboxRelayRunReport, OutboxState, OutboxWriter,
};

pub struct DefaultOutboxRelay<Uow, O, F, W, P>
where
    Uow: UnitOfWork,
    O: Outbox,
    F: OutboxFetcher<Uow = Uow, Outbox = O>,
    W: OutboxWriter<Uow = Uow, Outbox = O>,
    P: OutboxPublisher<Outbox = O>,
{
    config: OutboxRelayConfig,
    publisher: P,
    fetcher: F,
    writer: W,
    stop_requested: AtomicBool,
    _marker: PhantomData<fn() -> O>,
}

impl<Uow, O, F, W, P> DefaultOutboxRelay<Uow, O, F, W, P>
where
    Uow: UnitOfWork,
    O: Outbox,
    F: OutboxFetcher<Uow = Uow, Outbox = O>,
    W: OutboxWriter<Uow = Uow, Outbox = O>,
    P: OutboxPublisher<Outbox = O>,
{
    pub fn new(config: OutboxRelayConfig, publisher: P, fetcher: F, writer: W) -> Self {
        Self {
            config,
            publisher,
            fetcher,
            writer,
            stop_requested: AtomicBool::new(false),
            _marker: PhantomData,
        }
    }
}

impl<Uow, O, F, W, P> OutboxRelay for DefaultOutboxRelay<Uow, O, F, W, P>
where
    Uow: UnitOfWork,
    O: Outbox,
    F: OutboxFetcher<Uow = Uow, Outbox = O>,
    W: OutboxWriter<Uow = Uow, Outbox = O>,
    P: OutboxPublisher<Outbox = O>,
{
    type Uow = Uow;
    type Outbox = O;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(AtomicOrdering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, AtomicOrdering::SeqCst);
    }

    async fn run_forever(&self, uow: &mut Self::Uow) -> Result<(), OutboxRelayError> {
        let polling_options = &self.config.polling_options;
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

        let relay_instance = &self.config.instance;
        let lease_duration = self.config.lease_duration;
        let batch_size = self.config.batch_size;
        let retry_options = self.config.retry_options;

        uow.begin().await?;

        let operation_result = async {
            let mut outboxes = self.fetcher.fetch(uow, batch_size).await?;

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

            self.writer.write_outbox(uow, &outboxes).await?;

            Ok(outboxes)
        }
        .await;

        let mut outboxes = match operation_result {
            Ok(value) => {
                uow.commit().await?;
                value
            }
            Err(error) => return Err(uow.rollback_with_operation_error(error).await?),
        };

        if outboxes.is_empty() {
            return Ok(OutboxRelayRunReport::Idle);
        }

        let publish_results = self.publisher.publish_outbox(&outboxes).await?;

        for publish_result in publish_results {
            match publish_result {
                OutboxPublishResult::Success { input_index, .. } => {
                    outboxes[input_index].ack()?;
                }
                OutboxPublishResult::Failed {
                    input_index, cause, ..
                } => {
                    outboxes[input_index].nack(&cause, &retry_options)?;
                }
            }
        }

        let proceeded_outbox_count = outboxes.len().min(u32::MAX as usize) as u32;

        uow.begin().await?;

        let operation_result = async {
            self.writer.write_outbox(uow, &outboxes).await?;
            Ok(())
        }
        .await;

        match operation_result {
            Ok(()) => {
                uow.commit().await?;
            }
            Err(error) => return Err(uow.rollback_with_operation_error(error).await?),
        }

        Ok(OutboxRelayRunReport::Progress {
            proceeded_outbox_count,
        })
    }
}
