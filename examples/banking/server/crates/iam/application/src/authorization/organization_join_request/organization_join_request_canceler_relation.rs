use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationJoinRequestRequesterRelation;

/// Allows the requesting user to cancel their own join request.
pub struct OrganizationJoinRequestCancelerRelation;

impl Relation for OrganizationJoinRequestCancelerRelation {
    const NAME: RelationName = RelationName::new("canceler");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::ComputedUserset {
            relation: RelationNameOwned::from(OrganizationJoinRequestRequesterRelation::NAME),
        }
    }
}
