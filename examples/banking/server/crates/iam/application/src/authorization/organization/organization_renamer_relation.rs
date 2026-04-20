use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to rename an organization.
pub struct OrganizationRenamerRelation;

impl Relation for OrganizationRenamerRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("renamer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationAdminRelation::REF,
        },
    ]);
}
