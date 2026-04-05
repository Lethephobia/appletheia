use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Links a membership to its organization.
pub struct OrganizationMembershipOrganizationRelation;

impl Relation for OrganizationMembershipOrganizationRelation {
    const NAME: RelationName = RelationName::new("organization");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
