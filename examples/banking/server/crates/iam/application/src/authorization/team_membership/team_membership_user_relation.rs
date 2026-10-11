use crate::authorization::{
    OrganizationMembershipUserRelation, TeamMembershipOrganizationMembershipRelation,
};
use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_domain::TeamMembership;

pub struct TeamMembershipUserRelation;

impl Relation for TeamMembershipUserRelation {
    const REF: RelationRef = RelationRef::new(TeamMembership::TYPE, RelationName::new("user"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: TeamMembershipOrganizationMembershipRelation::REF.into(),
            computed_userset: OrganizationMembershipUserRelation::REF.into(),
        }
    }
}
