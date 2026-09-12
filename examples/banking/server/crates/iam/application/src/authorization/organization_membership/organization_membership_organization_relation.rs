use super::OrganizationMembershipOrganizationDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationMembership};

/// Links a membership to its organization.
pub struct OrganizationMembershipOrganizationRelation;

impl Relation for OrganizationMembershipOrganizationRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationMembership::TYPE,
        RelationName::new("organization"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            OrganizationMembership,
            _,
            OrganizationMembershipOrganizationDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<OrganizationMembership, Organization>(
                aggregate.aggregate_id(),
                *aggregate.organization_id()?,
            );
            Ok(entries)
        })
    }
}
