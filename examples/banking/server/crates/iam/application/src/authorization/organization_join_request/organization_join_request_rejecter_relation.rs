use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationJoinRequestOrganizationRelation;
use crate::OrganizationOwnerRelation;

/// Allows organization owners to reject join requests.
pub struct OrganizationJoinRequestRejecterRelation;

impl Relation for OrganizationJoinRequestRejecterRelation {
    const NAME: RelationName = RelationName::new("rejecter");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: RelationNameOwned::from(
                OrganizationJoinRequestOrganizationRelation::NAME,
            ),
            computed_relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
        }
    }
}
