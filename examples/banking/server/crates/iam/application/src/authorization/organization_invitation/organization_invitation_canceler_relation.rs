use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{OrganizationInvitation, OrganizationInvitationOrganizationRelation};
use crate::OrganizationInviterRelation;

/// Allows organization inviters to cancel invitations.
pub struct OrganizationInvitationCancelerRelation;

impl Relation for OrganizationInvitationCancelerRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationInvitation::TYPE, RelationName::new("canceler"));

    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: OrganizationInvitationOrganizationRelation::REF,
        computed_userset: OrganizationInviterRelation::REF,
    };
}
