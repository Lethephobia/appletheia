use appletheia_application::outbox::{DefaultOutboxRelay, OutboxRelayConfig};
use appletheia_application::outbox::command::CommandOutbox;

use crate::google_cloud::pubsub::outbox::command::PubsubCommandOutboxPublisher;
use crate::postgresql::outbox::command::{PgCommandOutboxFetcher, PgCommandOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub type PgPubsubCommandOutboxRelay = DefaultOutboxRelay<
    PgUnitOfWork,
    CommandOutbox,
    PgCommandOutboxFetcher,
    PgCommandOutboxWriter,
    PubsubCommandOutboxPublisher,
>;

pub fn pg_pubsub_command_outbox_relay(
    config: OutboxRelayConfig,
    publisher: PubsubCommandOutboxPublisher,
) -> PgPubsubCommandOutboxRelay {
    DefaultOutboxRelay::new(
        config,
        publisher,
        PgCommandOutboxFetcher::new(),
        PgCommandOutboxWriter::new(),
    )
}

pub fn pg_pubsub_command_outbox_relay_with_components(
    config: OutboxRelayConfig,
    publisher: PubsubCommandOutboxPublisher,
    fetcher: PgCommandOutboxFetcher,
    writer: PgCommandOutboxWriter,
) -> PgPubsubCommandOutboxRelay {
    DefaultOutboxRelay::new(config, publisher, fetcher, writer)
}
