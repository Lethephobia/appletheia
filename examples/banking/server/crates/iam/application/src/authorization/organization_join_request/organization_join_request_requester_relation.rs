use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::OrganizationJoinRequest;

/// Links a join request to the user who requested membership.
pub struct OrganizationJoinRequestRequesterRelation;

impl Relation for OrganizationJoinRequestRequesterRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationJoinRequest::TYPE,
        RelationName::new("requester"),
    );

    const EXPR: UsersetExpr = UsersetExpr::This;
}
