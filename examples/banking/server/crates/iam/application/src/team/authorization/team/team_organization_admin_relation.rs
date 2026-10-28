use crate::organization::authorization::OrganizationAdminRelation;
use crate::team::authorization::TeamOrganizationRelation;
use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_domain::Team;

pub struct TeamOrganizationAdminRelation;

impl Relation for TeamOrganizationAdminRelation {
    const REF: RelationRef = RelationRef::new(Team::TYPE, RelationName::new("organization_admin"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: TeamOrganizationRelation::REF.into(),
            computed_userset: OrganizationAdminRelation::REF.into(),
        }
    }
}
