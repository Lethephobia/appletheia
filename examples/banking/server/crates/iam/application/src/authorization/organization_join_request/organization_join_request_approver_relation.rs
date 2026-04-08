use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationJoinRequest, OrganizationJoinRequestOrganizationRelation};
use crate::OrganizationOwnerRelation;

/// Allows organization owners to approve join requests.
pub struct OrganizationJoinRequestApproverRelation;

impl Relation for OrganizationJoinRequestApproverRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationJoinRequest::TYPE, RelationName::new("approver"));

    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: OrganizationJoinRequestOrganizationRelation::REF,
        computed_userset: OrganizationOwnerRelation::REF,
    };
}
