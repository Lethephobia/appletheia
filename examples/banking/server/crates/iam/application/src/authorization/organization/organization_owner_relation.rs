use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::Organization;

/// Allows the owning subject itself.
pub struct OrganizationOwnerRelation;

impl Relation for OrganizationOwnerRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("owner"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
