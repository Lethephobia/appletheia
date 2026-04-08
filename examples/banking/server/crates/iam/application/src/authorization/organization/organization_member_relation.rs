use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows direct members and the owning subject to count as organization members.
pub struct OrganizationMemberRelation;

impl Relation for OrganizationMemberRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("member"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
