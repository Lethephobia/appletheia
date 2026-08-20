use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationMembership, OrganizationMembershipOrganizationRelation};
use crate::OrganizationAdminRelation;

/// Allows organization administrators to remove a membership.
pub struct OrganizationMembershipRemoverRelation;

impl Relation for OrganizationMembershipRemoverRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationMembership::TYPE, RelationName::new("remover"));

    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: OrganizationMembershipOrganizationRelation::REF,
        computed_userset: OrganizationAdminRelation::REF,
    };
}
