use super::TeamManagerDerivationHandlerError;
use crate::authorization::{
    OrganizationMemberRelation, OrganizationMembershipUserRelation, TeamOrganizationAdminRelation,
    TeamOrganizationRelation,
};
use appletheia::application::aggregate::AggregateRef;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, RelationshipEntry,
    RelationshipSubject, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationMembership, Team, TeamMembership};

pub struct TeamManagerRelation;

impl Relation for TeamManagerRelation {
    const REF: RelationRef = RelationRef::new(Team::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Intersection(vec![
            UsersetExpr::Union(vec![
                UsersetExpr::ComputedUserset {
                    relation: TeamOrganizationAdminRelation::REF.into(),
                },
                UsersetExpr::this::<TeamMembership, _, TeamManagerDerivationHandlerError>(
                    |aggregate| {
                        let mut entries = RelationshipEntries::new();
                        if aggregate.is_active()? && aggregate.is_manager()? {
                            entries.insert_entry(RelationshipEntry::from_parts(
                                AggregateRef::from_id::<Team>(*aggregate.team_id()?),
                                RelationshipSubject::aggregate_set::<OrganizationMembership>(
                                    *aggregate.organization_membership_id()?,
                                    OrganizationMembershipUserRelation::REF,
                                ),
                            ));
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
