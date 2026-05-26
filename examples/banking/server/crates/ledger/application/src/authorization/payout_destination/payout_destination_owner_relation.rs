use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::PayoutDestination;

/// Allows the owning subject itself.
pub struct PayoutDestinationOwnerRelation;

impl Relation for PayoutDestinationOwnerRelation {
    const REF: RelationRef = RelationRef::new(PayoutDestination::TYPE, RelationName::new("owner"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
