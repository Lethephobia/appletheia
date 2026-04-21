use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationJoinRequest, OrganizationJoinRequestOrganizationRelation};
use crate::OrganizationAdminRelation;

/// Allows organization administrators to reject join requests.
pub struct OrganizationJoinRequestRejecterRelation;

impl Relation for OrganizationJoinRequestRejecterRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationJoinRequest::TYPE, RelationName::new("rejecter"));

    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: OrganizationJoinRequestOrganizationRelation::REF,
        computed_userset: OrganizationAdminRelation::REF,
    };
}
