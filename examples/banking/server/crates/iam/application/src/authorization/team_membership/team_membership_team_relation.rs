use super::TeamMembershipTeamDerivationHandlerError;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Team, TeamMembership};

pub struct TeamMembershipTeamRelation;

impl Relation for TeamMembershipTeamRelation {
    const REF: RelationRef = RelationRef::new(TeamMembership::TYPE, RelationName::new("team"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<TeamMembership, _, TeamMembershipTeamDerivationHandlerError>(
            |aggregate| {
                let mut entries = RelationshipEntries::new();
                entries
                    .insert::<TeamMembership, Team>(aggregate.aggregate_id(), aggregate.team_id()?);
                Ok(entries)
            },
        )
    }
}
