use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationAdminRelation};

/// Allows organization administrators to invite users.
pub struct OrganizationInviterRelation;

impl Relation for OrganizationInviterRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("inviter"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationAdminRelation::REF,
        },
    ]);
}
