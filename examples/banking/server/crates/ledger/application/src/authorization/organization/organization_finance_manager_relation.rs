use super::OrganizationFinanceManagerDerivationHandlerError;
use appletheia::application::aggregate::AggregateRef;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, RelationshipEntry,
    RelationshipSubject, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_application::authorization::OrganizationAdminRelation;
use banking_iam_application::authorization::{
    OrganizationMembershipUserRelation, TeamMemberRelation,
};
use banking_iam_domain::{Organization, OrganizationMembership, Team};
use banking_ledger_domain::{
    OrganizationLedgerRole, OrganizationLedgerRoleAssignee, OrganizationLedgerRoleAssignment,
};

pub struct OrganizationFinanceManagerRelation;
impl Relation for OrganizationFinanceManagerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("finance_manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::ComputedUserset {
                relation: OrganizationAdminRelation::REF.into(),
            },
            UsersetExpr::this::<
                OrganizationLedgerRoleAssignment,
                _,
                OrganizationFinanceManagerDerivationHandlerError,
            >(|assignment| {
                let mut entries = RelationshipEntries::new();
                if assignment.is_active()?
                    && assignment.role()? == OrganizationLedgerRole::FinanceManager
                {
                    let subject = match assignment.assignee()? {
                        OrganizationLedgerRoleAssignee::OrganizationMembership(id) => {
                            RelationshipSubject::aggregate_set::<OrganizationMembership>(
                                id,
                                OrganizationMembershipUserRelation::REF,
                            )
                        }
                        OrganizationLedgerRoleAssignee::Team(id) => {
                            RelationshipSubject::aggregate_set::<Team>(id, TeamMemberRelation::REF)
                        }
                    };
                    entries.insert_entry(RelationshipEntry::from_parts(
                        AggregateRef::from_id::<Organization>(assignment.organization_id()?),
                        subject,
                    ));
                }
                Ok(entries)
            }),
        ])
    }
}
