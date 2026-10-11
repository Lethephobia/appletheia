use super::OrganizationRoleAssignmentOrganizationRelation;
use crate::authorization::OrganizationAdminRelation;
use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_domain::OrganizationRoleAssignment;

pub struct OrganizationRoleAssignmentManagerRelation;

impl Relation for OrganizationRoleAssignmentManagerRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationRoleAssignment::TYPE,
        RelationName::new("manager"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: OrganizationRoleAssignmentOrganizationRelation::REF.into(),
            computed_userset: OrganizationAdminRelation::REF.into(),
        }
    }
}
