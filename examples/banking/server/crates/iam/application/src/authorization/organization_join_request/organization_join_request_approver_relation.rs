use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationJoinRequest, OrganizationJoinRequestOrganizationRelation};
use crate::OrganizationAdminRelation;

/// Allows organization administrators to approve join requests.
pub struct OrganizationJoinRequestApproverRelation;

impl Relation for OrganizationJoinRequestApproverRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationJoinRequest::TYPE, RelationName::new("approver"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: OrganizationJoinRequestOrganizationRelation::REF.into(),
            computed_userset: OrganizationAdminRelation::REF.into(),
        }
    }
}
