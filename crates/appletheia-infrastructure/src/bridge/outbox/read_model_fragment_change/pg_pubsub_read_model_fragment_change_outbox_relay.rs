use appletheia_application::outbox::DefaultOutboxRelay;
use appletheia_application::outbox::read_model_fragment_change::ReadModelFragmentChangeOutbox;

use crate::google_cloud::pubsub::messaging::PubsubReadModelFragmentChangePublisher;
use crate::postgresql::outbox::read_model_fragment_change::{
    PgReadModelFragmentChangeOutboxFetcher, PgReadModelFragmentChangeOutboxWriter,
};
use crate::postgresql::unit_of_work::PgUnitOfWorkFactory;

/// Relays PostgreSQL source-fragment changes through Google Cloud Pub/Sub.
pub type PgPubsubReadModelFragmentChangeOutboxRelay = DefaultOutboxRelay<
    PgUnitOfWorkFactory,
    ReadModelFragmentChangeOutbox,
    PgReadModelFragmentChangeOutboxFetcher,
    PgReadModelFragmentChangeOutboxWriter,
    PubsubReadModelFragmentChangePublisher,
>;
