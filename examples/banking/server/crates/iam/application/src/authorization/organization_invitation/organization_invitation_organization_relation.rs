use super::OrganizationInvitationOrganizationDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationInvitation};

/// Links an invitation to its organization.
pub struct OrganizationInvitationOrganizationRelation;

impl Relation for OrganizationInvitationOrganizationRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationInvitation::TYPE,
        RelationName::new("organization"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            OrganizationInvitation,
            _,
            OrganizationInvitationOrganizationDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<OrganizationInvitation, Organization>(
                aggregate.aggregate_id(),
                *aggregate.organization_id()?,
            );
            Ok(entries)
        })
    }
}
