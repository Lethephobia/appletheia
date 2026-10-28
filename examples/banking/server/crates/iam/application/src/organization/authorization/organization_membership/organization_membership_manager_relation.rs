use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_domain::OrganizationMembership;

use super::OrganizationMembershipOrganizationRelation;
use crate::organization::authorization::OrganizationAdminRelation;

/// Allows organization admins, including the owner, to manage memberships.
pub struct OrganizationMembershipManagerRelation;

impl Relation for OrganizationMembershipManagerRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationMembership::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: OrganizationMembershipOrganizationRelation::REF.into(),
            computed_userset: OrganizationAdminRelation::REF.into(),
        }
    }
}
