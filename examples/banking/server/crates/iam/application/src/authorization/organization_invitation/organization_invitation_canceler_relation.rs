use appletheia::application::authorization::{
    Relation, RelationName, RelationNameOwned, UsersetExpr,
};

use super::OrganizationInvitationOrganizationRelation;
use crate::OrganizationInviterRelation;

/// Allows organization inviters to cancel invitations.
pub struct OrganizationInvitationCancelerRelation;

impl Relation for OrganizationInvitationCancelerRelation {
    const NAME: RelationName = RelationName::new("canceler");

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: RelationNameOwned::from(
                OrganizationInvitationOrganizationRelation::NAME,
            ),
            computed_relation: RelationNameOwned::from(OrganizationInviterRelation::NAME),
        }
    }
}
