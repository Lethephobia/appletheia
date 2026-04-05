use appletheia::application::authorization::{Relation, RelationName, UsersetExpr};

/// Links an invitation to its organization.
pub struct OrganizationInvitationOrganizationRelation;

impl Relation for OrganizationInvitationOrganizationRelation {
    const NAME: RelationName = RelationName::new("organization");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::This
    }
}
