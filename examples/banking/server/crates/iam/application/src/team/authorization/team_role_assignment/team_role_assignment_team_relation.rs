use super::TeamRoleAssignmentTeamDerivationHandlerError;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::Team;
use banking_iam_domain::TeamRoleAssignment;

pub struct TeamRoleAssignmentTeamRelation;

impl Relation for TeamRoleAssignmentTeamRelation {
    const REF: RelationRef = RelationRef::new(TeamRoleAssignment::TYPE, RelationName::new("team"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<TeamRoleAssignment, _, TeamRoleAssignmentTeamDerivationHandlerError>(
            |aggregate| {
                let mut entries = RelationshipEntries::new();
                entries.insert::<TeamRoleAssignment, Team>(
                    aggregate.aggregate_id(),
                    aggregate.team_id()?,
                );
                Ok(entries)
            },
        )
    }
}
