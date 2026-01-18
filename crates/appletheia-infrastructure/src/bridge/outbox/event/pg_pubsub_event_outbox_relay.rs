use std::sync::atomic::{AtomicBool, Ordering};

use appletheia_application::outbox::{
    OutboxRelay, OutboxRelayConfig, OutboxRelayConfigAccess, event::EventOutbox,
};

use crate::google_cloud::pubsub::outbox::event::PubsubEventOutboxPublisher;
use crate::postgresql::outbox::event::{PgEventOutboxFetcher, PgEventOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgPubsubEventOutboxRelay {
    config: OutboxRelayConfig,
    publisher: PubsubEventOutboxPublisher,
    fetcher: PgEventOutboxFetcher,
    writer: PgEventOutboxWriter,
    stop_requested: AtomicBool,
}

impl PgPubsubEventOutboxRelay {
    pub fn new(config: OutboxRelayConfig, publisher: PubsubEventOutboxPublisher) -> Self {
        Self {
            config,
            publisher,
            fetcher: PgEventOutboxFetcher::new(),
            writer: PgEventOutboxWriter::new(),
            stop_requested: AtomicBool::new(false),
        }
    }

    pub fn with_components(
        config: OutboxRelayConfig,
        publisher: PubsubEventOutboxPublisher,
        fetcher: PgEventOutboxFetcher,
        writer: PgEventOutboxWriter,
    ) -> Self {
        Self {
            config,
            publisher,
            fetcher,
            writer,
            stop_requested: AtomicBool::new(false),
        }
    }
}

impl OutboxRelayConfigAccess for PgPubsubEventOutboxRelay {
    fn config(&self) -> &OutboxRelayConfig {
        &self.config
    }
}

impl OutboxRelay for PgPubsubEventOutboxRelay {
    type Uow = PgUnitOfWork;
    type Outbox = EventOutbox;

    type Fetcher = PgEventOutboxFetcher;
    type Writer = PgEventOutboxWriter;
    type Publisher = PubsubEventOutboxPublisher;

    fn outbox_fetcher(&self) -> &Self::Fetcher {
        &self.fetcher
    }

    fn outbox_writer(&self) -> &Self::Writer {
        &self.writer
    }

    fn outbox_publisher(&self) -> &Self::Publisher {
        &self.publisher
    }

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(Ordering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, Ordering::SeqCst);
    }
}
