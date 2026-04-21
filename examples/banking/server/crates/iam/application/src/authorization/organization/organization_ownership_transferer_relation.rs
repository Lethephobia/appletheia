use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows current organization owners to transfer ownership.
pub struct OrganizationOwnershipTransfererRelation;

impl Relation for OrganizationOwnershipTransfererRelation {
    const REF: RelationRef = RelationRef::new(
        Organization::TYPE,
        RelationName::new("ownership_transferer"),
    );

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationOwnerRelation::REF,
    };
}
