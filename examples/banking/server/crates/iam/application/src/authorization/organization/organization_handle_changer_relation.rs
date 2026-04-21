use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to change an organization handle.
pub struct OrganizationHandleChangerRelation;

impl Relation for OrganizationHandleChangerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("handle_changer"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationAdminRelation::REF,
    };
}
