use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationJoinRequestOrganizationRelation;
use crate::OrganizationOwnerRelation;

/// Allows organization owners to approve join requests.
pub struct OrganizationJoinRequestApproverRelation;

impl Relation for OrganizationJoinRequestApproverRelation {
    const NAME: RelationName = RelationName::new("approver");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: RelationNameOwned::from(
                OrganizationJoinRequestOrganizationRelation::NAME,
            ),
            computed_relation: RelationNameOwned::from(OrganizationOwnerRelation::NAME),
        }
    }
}
