use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::OrganizationMembership;

/// Links a membership to its organization.
pub struct OrganizationMembershipOrganizationRelation;

impl Relation for OrganizationMembershipOrganizationRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationMembership::TYPE,
        RelationName::new("organization"),
    );

    const EXPR: UsersetExpr = UsersetExpr::This;
}
