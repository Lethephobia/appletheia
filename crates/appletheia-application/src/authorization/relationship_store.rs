use crate::event::AggregateTypeOwned;
use crate::unit_of_work::UnitOfWork;

use super::RelationshipStoreError;
use super::{AggregateRef, RelationRefOwned, RelationshipChange, RelationshipSubject};

#[allow(async_fn_in_trait)]
pub trait RelationshipStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn apply_changes(
        &self,
        uow: &mut Self::Uow,
        changes: &[RelationshipChange],
    ) -> Result<(), RelationshipStoreError>;

    async fn read_aggregates_by_subject(
        &self,
        uow: &mut Self::Uow,
        subject: &RelationshipSubject,
        relation: &RelationRefOwned,
    ) -> Result<Vec<AggregateRef>, RelationshipStoreError>;

    async fn read_subjects_by_aggregate(
        &self,
        uow: &mut Self::Uow,
        aggregate: &AggregateRef,
        relation: &RelationRefOwned,
        subject_aggregate_type: Option<&AggregateTypeOwned>,
    ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError>;
}
