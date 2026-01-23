use appletheia_application::command::CommandName;
use appletheia_application::outbox::command::CommandOutbox;
use appletheia_application::outbox::{DefaultOutboxRelay, OutboxRelayConfig};

use crate::google_cloud::pubsub::outbox::command::PubsubCommandOutboxPublisher;
use crate::postgresql::outbox::command::{PgCommandOutboxFetcher, PgCommandOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWork;

pub type PgPubsubCommandOutboxRelay<CN> = DefaultOutboxRelay<
    PgUnitOfWork,
    CommandOutbox<CN>,
    PgCommandOutboxFetcher<CN>,
    PgCommandOutboxWriter<CN>,
    PubsubCommandOutboxPublisher<CN>,
>;

pub fn pg_pubsub_command_outbox_relay<CN: CommandName>(
    config: OutboxRelayConfig,
    publisher: PubsubCommandOutboxPublisher<CN>,
) -> PgPubsubCommandOutboxRelay<CN> {
    DefaultOutboxRelay::new(
        config,
        publisher,
        PgCommandOutboxFetcher::<CN>::new(),
        PgCommandOutboxWriter::<CN>::new(),
    )
}

pub fn pg_pubsub_command_outbox_relay_with_components<CN: CommandName>(
    config: OutboxRelayConfig,
    publisher: PubsubCommandOutboxPublisher<CN>,
    fetcher: PgCommandOutboxFetcher<CN>,
    writer: PgCommandOutboxWriter<CN>,
) -> PgPubsubCommandOutboxRelay<CN> {
    DefaultOutboxRelay::new(config, publisher, fetcher, writer)
}
