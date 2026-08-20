use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationMembership, OrganizationMembershipOrganizationRelation};
use crate::OrganizationAdminRelation;

/// Allows organization administrators to change membership roles.
pub struct OrganizationMembershipRolesChangerRelation;

impl Relation for OrganizationMembershipRolesChangerRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationMembership::TYPE,
        RelationName::new("roles_changer"),
    );

    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: OrganizationMembershipOrganizationRelation::REF,
        computed_userset: OrganizationAdminRelation::REF,
    };
}
