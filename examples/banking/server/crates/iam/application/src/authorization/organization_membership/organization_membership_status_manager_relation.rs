use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationMembershipOrganizationRelation;
use crate::OrganizationOwnerRelation;

/// Allows organization owners to manage membership status transitions.
pub struct OrganizationMembershipStatusManagerRelation;

impl Relation for OrganizationMembershipStatusManagerRelation {
    const NAME: RelationName = RelationName::new("status_manager");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: RelationNameOwned::from(
                OrganizationMembershipOrganizationRelation::NAME,
            ),
            computed_relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
        }
    }
}
