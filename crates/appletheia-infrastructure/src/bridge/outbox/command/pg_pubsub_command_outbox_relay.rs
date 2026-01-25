use appletheia_application::outbox::DefaultOutboxRelay;
use appletheia_application::outbox::command::CommandOutbox;

use crate::google_cloud::pubsub::outbox::command::PubsubCommandOutboxPublisher;
use crate::postgresql::outbox::command::{PgCommandOutboxFetcher, PgCommandOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;

pub type PgPubsubCommandOutboxRelay = DefaultOutboxRelay<
    PgUnitOfWorkFactory,
    CommandOutbox,
    PgCommandOutboxFetcher,
    PgCommandOutboxWriter,
    PubsubCommandOutboxPublisher,
>;
