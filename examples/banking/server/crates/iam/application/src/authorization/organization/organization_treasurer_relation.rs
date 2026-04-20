use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows elevated treasurers and the owner.
pub struct OrganizationTreasurerRelation;

impl Relation for OrganizationTreasurerRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("treasurer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
