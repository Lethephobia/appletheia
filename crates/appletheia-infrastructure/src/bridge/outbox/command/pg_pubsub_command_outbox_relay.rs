use std::sync::atomic::{AtomicBool, Ordering};

use appletheia_application::outbox::{
    OutboxRelayConfig, OutboxRelayConfigAccess,
    command::{
        CommandOutboxFetcherAccess, CommandOutboxPublisherAccess, CommandOutboxRelay,
        CommandOutboxWriterAccess,
    },
};

use crate::google_cloud::pubsub::outbox::command::PubsubCommandOutboxPublisher;
use crate::postgresql::outbox::command::{PgCommandOutboxFetcher, PgCommandOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgPubsubCommandOutboxRelay {
    config: OutboxRelayConfig,
    publisher: PubsubCommandOutboxPublisher,
    fetcher: PgCommandOutboxFetcher,
    writer: PgCommandOutboxWriter,
    stop_requested: AtomicBool,
}

impl PgPubsubCommandOutboxRelay {
    pub fn new(config: OutboxRelayConfig, publisher: PubsubCommandOutboxPublisher) -> Self {
        Self {
            config,
            publisher,
            fetcher: PgCommandOutboxFetcher::new(),
            writer: PgCommandOutboxWriter::new(),
            stop_requested: AtomicBool::new(false),
        }
    }

    pub fn with_components(
        config: OutboxRelayConfig,
        publisher: PubsubCommandOutboxPublisher,
        fetcher: PgCommandOutboxFetcher,
        writer: PgCommandOutboxWriter,
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

impl OutboxRelayConfigAccess for PgPubsubCommandOutboxRelay {
    fn config(&self) -> &OutboxRelayConfig {
        &self.config
    }
}

impl CommandOutboxPublisherAccess for PgPubsubCommandOutboxRelay {
    type Publisher = PubsubCommandOutboxPublisher;

    fn outbox_publisher(&self) -> &Self::Publisher {
        &self.publisher
    }
}

impl CommandOutboxFetcherAccess for PgPubsubCommandOutboxRelay {
    type Fetcher = PgCommandOutboxFetcher;

    fn outbox_fetcher(&self) -> &Self::Fetcher {
        &self.fetcher
    }
}

impl CommandOutboxWriterAccess for PgPubsubCommandOutboxRelay {
    type Writer = PgCommandOutboxWriter;

    fn outbox_writer(&self) -> &Self::Writer {
        &self.writer
    }
}

impl CommandOutboxRelay for PgPubsubCommandOutboxRelay {
    type Uow = PgUnitOfWork;

    fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(Ordering::SeqCst)
    }

    fn request_graceful_stop(&mut self) {
        self.stop_requested.store(true, Ordering::SeqCst);
    }
}
