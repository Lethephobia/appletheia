use appletheia_application::outbox::DefaultOutboxRelay;
use appletheia_application::outbox::read_model_invalidation::ReadModelInvalidationOutbox;

use crate::google_cloud::pubsub::messaging::PubsubCloudEventPublisher;
use crate::postgresql::outbox::read_model_invalidation::{
    PgReadModelInvalidationOutboxFetcher, PgReadModelInvalidationOutboxWriter,
};
use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;
use appletheia_application::messaging::ReadModelInvalidationPublisher;

/// Relays PostgreSQL read-model invalidations through Google Cloud Pub/Sub.
pub type PgPubsubReadModelInvalidationOutboxRelay = DefaultOutboxRelay<
    PgUnitOfWorkFactory,
    ReadModelInvalidationOutbox,
    PgReadModelInvalidationOutboxFetcher,
    PgReadModelInvalidationOutboxWriter,
    ReadModelInvalidationPublisher<PubsubCloudEventPublisher>,
>;
