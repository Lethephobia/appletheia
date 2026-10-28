use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::OrganizationInvitationOrganizationRelation;
use crate::organization::authorization::OrganizationAdminRelation;
use banking_iam_domain::OrganizationInvitation;

/// Allows organization admins to manage invitations.
pub struct OrganizationInvitationManagerRelation;

impl Relation for OrganizationInvitationManagerRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationInvitation::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: OrganizationInvitationOrganizationRelation::REF.into(),
            computed_userset: OrganizationAdminRelation::REF.into(),
        }
    }
}
