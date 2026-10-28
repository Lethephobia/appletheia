use crate::team::authorization::{TeamManagerRelation, TeamMembershipTeamRelation};
use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_domain::TeamMembership;

pub struct TeamMembershipManagerRelation;

impl Relation for TeamMembershipManagerRelation {
    const REF: RelationRef = RelationRef::new(TeamMembership::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: TeamMembershipTeamRelation::REF.into(),
            computed_userset: TeamManagerRelation::REF.into(),
        }
    }
}
