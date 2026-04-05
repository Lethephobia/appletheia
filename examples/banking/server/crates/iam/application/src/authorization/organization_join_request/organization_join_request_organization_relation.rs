use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Links a join request to its organization.
pub struct OrganizationJoinRequestOrganizationRelation;

impl Relation for OrganizationJoinRequestOrganizationRelation {
    const NAME: RelationName = RelationName::new("organization");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
