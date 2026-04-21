use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationMembership, OrganizationMembershipOrganizationRelation};
use crate::OrganizationAdminRelation;

/// Allows organization administrators to manage membership status transitions.
pub struct OrganizationMembershipStatusManagerRelation;

impl Relation for OrganizationMembershipStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationMembership::TYPE,
        RelationName::new("status_manager"),
    );

    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: OrganizationMembershipOrganizationRelation::REF,
        computed_userset: OrganizationAdminRelation::REF,
    };
}
