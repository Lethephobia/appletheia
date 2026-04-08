use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::OrganizationJoinRequest;

/// Links a join request to its organization.
pub struct OrganizationJoinRequestOrganizationRelation;

impl Relation for OrganizationJoinRequestOrganizationRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationJoinRequest::TYPE,
        RelationName::new("organization"),
    );

    const EXPR: UsersetExpr = UsersetExpr::This;
}
