use crate::aggregate::AggregateTypeOwned;
use crate::unit_of_work::UnitOfWork;

use super::RelationshipStoreError;
use super::{RelationRefOwned, Relationship, RelationshipSubject};
use crate::aggregate::AggregateRef;

#[allow(async_fn_in_trait)]
pub trait RelationshipStore: Send + Sync {
    type Uow: UnitOfWork;

    /// Replaces the complete set derived from one source in the caller's transaction.
    ///
    /// Call after successfully appending source events in the same transaction.
    /// The caller must enforce source write concurrency through the event store.
    async fn replace(
        &self,
        uow: &mut Self::Uow,
        source: &AggregateRef,
        relationships: &[Relationship],
    ) -> Result<(), RelationshipStoreError>;

    async fn read_targets_by_subject(
        &self,
        uow: &mut Self::Uow,
        subject: &RelationshipSubject,
        relation: &RelationRefOwned,
    ) -> Result<Vec<AggregateRef>, RelationshipStoreError>;

    async fn read_subjects_by_target(
        &self,
        uow: &mut Self::Uow,
        target: &AggregateRef,
        relation: &RelationRefOwned,
        subject_aggregate_type: Option<&AggregateTypeOwned>,
    ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError>;
}
