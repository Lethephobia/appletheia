use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows elevated organization administrators and the owner.
pub struct OrganizationAdminRelation;

impl Relation for OrganizationAdminRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("organization_admin"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
