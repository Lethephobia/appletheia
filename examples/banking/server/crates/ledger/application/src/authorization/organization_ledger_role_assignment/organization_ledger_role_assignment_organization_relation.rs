use super::OrganizationLedgerRoleAssignmentOrganizationDerivationHandlerError;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::Organization;
use banking_ledger_domain::OrganizationLedgerRoleAssignment;

pub struct OrganizationLedgerRoleAssignmentOrganizationRelation;

impl Relation for OrganizationLedgerRoleAssignmentOrganizationRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationLedgerRoleAssignment::TYPE,
        RelationName::new("organization"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            OrganizationLedgerRoleAssignment,
            _,
            OrganizationLedgerRoleAssignmentOrganizationDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<OrganizationLedgerRoleAssignment, Organization>(
                aggregate.aggregate_id(),
                aggregate.organization_id()?,
            );
            Ok(entries)
        })
    }
}
