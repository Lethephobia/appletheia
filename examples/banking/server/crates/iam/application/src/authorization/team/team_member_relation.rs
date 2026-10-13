use super::TeamMemberDerivationHandlerError;
use crate::authorization::{
    OrganizationMemberRelation, OrganizationMembershipUserRelation, TeamOrganizationRelation,
};
use appletheia::application::aggregate::AggregateRef;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, RelationshipEntry,
    RelationshipSubject, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationMembership, Team, TeamMembership};

pub struct TeamMemberRelation;

impl Relation for TeamMemberRelation {
    const REF: RelationRef = RelationRef::new(Team::TYPE, RelationName::new("member"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Intersection(vec![
            UsersetExpr::this::<TeamMembership, _, TeamMemberDerivationHandlerError>(|aggregate| {
                let mut entries = RelationshipEntries::new();
                if aggregate.is_active()? {
                    entries.insert_entry(RelationshipEntry::from_parts(
                        AggregateRef::from_id::<Team>(aggregate.team_id()?),
                        RelationshipSubject::aggregate_set::<OrganizationMembership>(
                            aggregate.organization_membership_id()?,
                            OrganizationMembershipUserRelation::REF,
                        ),
                    ));
                }
                Ok(entries)
            }),
            UsersetExpr::TupleToUserset {
                tupleset_relation: TeamOrganizationRelation::REF.into(),
                computed_userset: OrganizationMemberRelation::REF.into(),
            },
        ])
    }
}
