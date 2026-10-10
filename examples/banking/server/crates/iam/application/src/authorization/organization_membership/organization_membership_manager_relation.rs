use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationMembership, OrganizationMembershipOrganizationRelation};
use crate::OrganizationOwnerRelation;

/// Allows organization owners to manage memberships.
pub struct OrganizationMembershipManagerRelation;

impl Relation for OrganizationMembershipManagerRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationMembership::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: OrganizationMembershipOrganizationRelation::REF.into(),
            computed_userset: OrganizationOwnerRelation::REF.into(),
        }
    }
}
