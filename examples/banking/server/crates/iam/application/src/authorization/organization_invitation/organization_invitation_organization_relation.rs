use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::OrganizationInvitation;

/// Links an invitation to its organization.
pub struct OrganizationInvitationOrganizationRelation;

impl Relation for OrganizationInvitationOrganizationRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationInvitation::TYPE,
        RelationName::new("organization"),
    );

    const EXPR: UsersetExpr = UsersetExpr::This;
}
