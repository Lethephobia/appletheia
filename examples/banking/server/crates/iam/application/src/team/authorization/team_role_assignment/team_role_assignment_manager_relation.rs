use super::TeamRoleAssignmentTeamRelation;
use crate::team::authorization::TeamManagerRelation;
use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_domain::TeamRoleAssignment;

pub struct TeamRoleAssignmentManagerRelation;

impl Relation for TeamRoleAssignmentManagerRelation {
    const REF: RelationRef =
        RelationRef::new(TeamRoleAssignment::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: TeamRoleAssignmentTeamRelation::REF.into(),
            computed_userset: TeamManagerRelation::REF.into(),
        }
    }
}
