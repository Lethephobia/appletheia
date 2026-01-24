use appletheia_application::outbox::command::CommandOutbox;
use appletheia_application::outbox::DefaultOutboxRelay;

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
