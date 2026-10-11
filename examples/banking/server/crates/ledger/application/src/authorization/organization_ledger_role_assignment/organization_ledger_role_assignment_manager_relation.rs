use super::OrganizationLedgerRoleAssignmentOrganizationRelation;
use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::authorization::OrganizationAdminRelation;
use banking_ledger_domain::OrganizationLedgerRoleAssignment;

pub struct OrganizationLedgerRoleAssignmentManagerRelation;

impl Relation for OrganizationLedgerRoleAssignmentManagerRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationLedgerRoleAssignment::TYPE,
        RelationName::new("manager"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: OrganizationLedgerRoleAssignmentOrganizationRelation::REF.into(),
            computed_userset: OrganizationAdminRelation::REF.into(),
        }
    }
}
