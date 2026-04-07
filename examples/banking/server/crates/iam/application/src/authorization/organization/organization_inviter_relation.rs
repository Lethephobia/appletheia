use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{Organization, OrganizationOwnerRelation};

/// Allows owners to invite users to an organization.
pub struct OrganizationInviterRelation;

impl Relation for OrganizationInviterRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("inviter"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: OrganizationOwnerRelation::REF,
        },
    ]);
}
