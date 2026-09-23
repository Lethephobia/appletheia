use appletheia_application::outbox::DefaultOutboxRelay;
use appletheia_application::outbox::command::CommandOutbox;

use crate::google_cloud::pubsub::messaging::PubsubCloudEventPublisher;
use crate::postgresql::outbox::command::{PgCommandOutboxFetcher, PgCommandOutboxWriter};
use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;
use appletheia_application::messaging::CommandPublisher;

pub type PgPubsubCommandOutboxRelay = DefaultOutboxRelay<
    PgUnitOfWorkFactory,
    CommandOutbox,
    PgCommandOutboxFetcher,
    PgCommandOutboxWriter,
    CommandPublisher<PubsubCloudEventPublisher>,
>;
