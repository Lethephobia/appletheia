use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationJoinRequest, OrganizationJoinRequestRequesterRelation};

/// Allows the requesting user to cancel their own join request.
pub struct OrganizationJoinRequestCancelerRelation;

impl Relation for OrganizationJoinRequestCancelerRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationJoinRequest::TYPE, RelationName::new("canceler"));

    const EXPR: UsersetExpr = UsersetExpr::ComputedUserset {
        relation: OrganizationJoinRequestRequesterRelation::REF,
    };
}
