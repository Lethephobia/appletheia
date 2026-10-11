use super::TeamMembershipOrganizationMembershipDerivationHandlerError;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationMembership, TeamMembership};

pub struct TeamMembershipOrganizationMembershipRelation;

impl Relation for TeamMembershipOrganizationMembershipRelation {
    const REF: RelationRef = RelationRef::new(
        TeamMembership::TYPE,
        RelationName::new("organization_membership"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            TeamMembership,
            _,
            TeamMembershipOrganizationMembershipDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            if aggregate.is_active()? {
                entries.insert::<TeamMembership, OrganizationMembership>(
                    aggregate.aggregate_id(),
                    *aggregate.organization_membership_id()?,
                );
            }
            Ok(entries)
        })
    }
}
