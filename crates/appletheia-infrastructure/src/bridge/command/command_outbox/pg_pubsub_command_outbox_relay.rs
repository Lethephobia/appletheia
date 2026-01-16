use std::sync::atomic::{AtomicBool, Ordering};

use appletheia_application::command::{
    CommandOutboxFetcherAccess, CommandOutboxPublisherAccess, CommandOutboxRelay,
    CommandOutboxRelayConfig, CommandOutboxRelayConfigAccess, CommandOutboxWriterAccess,
};

use crate::google_cloud::pubsub::PubsubCommandOutboxPublisher;
use crate::postgresql::command_outbox::{PgCommandOutboxFetcher, PgCommandOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub struct PgPubsubCommandOutboxRelay {
    config: CommandOutboxRelayConfig,
    publisher: PubsubCommandOutboxPublisher,
    fetcher: PgCommandOutboxFetcher,
    writer: PgCommandOutboxWriter,
    stop_requested: AtomicBool,
}

impl PgPubsubCommandOutboxRelay {
    pub fn new(config: CommandOutboxRelayConfig, publisher: PubsubCommandOutboxPublisher) -> Self {
        Self {
            config,
            publisher,
            fetcher: PgCommandOutboxFetcher::new(),
            writer: PgCommandOutboxWriter::new(),
            stop_requested: AtomicBool::new(false),
        }
    }

    pub fn with_components(
        config: CommandOutboxRelayConfig,
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

impl CommandOutboxRelayConfigAccess for PgPubsubCommandOutboxRelay {
    fn config(&self) -> &CommandOutboxRelayConfig {
        &self.config
    }
}

impl CommandOutboxPublisherAccess for PgPubsubCommandOutboxRelay {
    type CommandOutboxPublisher = PubsubCommandOutboxPublisher;

    fn command_outbox_publisher(&self) -> &Self::CommandOutboxPublisher {
        &self.publisher
    }
}

impl CommandOutboxFetcherAccess for PgPubsubCommandOutboxRelay {
    type Fetcher = PgCommandOutboxFetcher;

    fn command_outbox_fetcher(&self) -> &Self::Fetcher {
        &self.fetcher
    }
}

impl CommandOutboxWriterAccess for PgPubsubCommandOutboxRelay {
    type Writer = PgCommandOutboxWriter;

    fn command_outbox_writer(&self) -> &Self::Writer {
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
