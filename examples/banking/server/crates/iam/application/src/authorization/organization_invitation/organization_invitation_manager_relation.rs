use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationInvitation, OrganizationInvitationOrganizationRelation};
use crate::OrganizationOwnerRelation;

/// Allows organization owners to manage invitations.
pub struct OrganizationInvitationManagerRelation;

impl Relation for OrganizationInvitationManagerRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationInvitation::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: OrganizationInvitationOrganizationRelation::REF.into(),
            computed_userset: OrganizationOwnerRelation::REF.into(),
        }
    }
}
