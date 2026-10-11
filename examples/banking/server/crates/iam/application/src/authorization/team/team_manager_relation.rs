use super::TeamManagerDerivationHandlerError;
use crate::authorization::{
    OrganizationMemberRelation, TeamMembershipUserRelation, TeamOrganizationAdminRelation,
    TeamOrganizationRelation,
};
use appletheia::application::aggregate::AggregateRef;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, RelationshipEntry,
    RelationshipSubject, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Team, TeamMembership, TeamRole, TeamRoleAssignee, TeamRoleAssignment};

pub struct TeamManagerRelation;

impl Relation for TeamManagerRelation {
    const REF: RelationRef = RelationRef::new(Team::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Intersection(vec![
            UsersetExpr::Union(vec![
                UsersetExpr::ComputedUserset {
                    relation: TeamOrganizationAdminRelation::REF.into(),
                },
                UsersetExpr::this::<TeamRoleAssignment, _, TeamManagerDerivationHandlerError>(
                    |assignment| {
                        let mut entries = RelationshipEntries::new();
                        if assignment.is_active()? && assignment.role()? == TeamRole::Manager {
                            match assignment.assignee()? {
                                TeamRoleAssignee::TeamMembership(id) => {
                                    entries.insert_entry(RelationshipEntry::from_parts(
                                        AggregateRef::from_id::<Team>(assignment.team_id()?),
                                        RelationshipSubject::aggregate_set::<TeamMembership>(
                                            id,
                                            TeamMembershipUserRelation::REF,
                                        ),
                                    ));
                                }
                            }
                        }
                        Ok(entries)
                    },
                ),
            ]),
            UsersetExpr::TupleToUserset {
                tupleset_relation: TeamOrganizationRelation::REF.into(),
                computed_userset: OrganizationMemberRelation::REF.into(),
            },
        ])
    }
}
