use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Links a join request to the user who requested membership.
pub struct OrganizationJoinRequestRequesterRelation;

impl Relation for OrganizationJoinRequestRequesterRelation {
    const NAME: RelationName = RelationName::new("requester");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
