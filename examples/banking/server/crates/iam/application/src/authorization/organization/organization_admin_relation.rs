use super::OrganizationAdminDerivationHandlerError;
use crate::authorization::{OrganizationMembershipUserRelation, OrganizationOwnerRelation};
use appletheia::application::aggregate::AggregateRef;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, RelationshipEntry,
    RelationshipSubject, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{
    Organization, OrganizationMembership, OrganizationRole, OrganizationRoleAssignee,
    OrganizationRoleAssignment,
};

pub struct OrganizationAdminRelation;

impl Relation for OrganizationAdminRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("admin"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::ComputedUserset {
                relation: OrganizationOwnerRelation::REF.into(),
            },
            UsersetExpr::this::<
                OrganizationRoleAssignment,
                _,
                OrganizationAdminDerivationHandlerError,
            >(|assignment| {
                let mut entries = RelationshipEntries::new();
                if assignment.is_active()? && assignment.role()? == OrganizationRole::Admin {
                    match assignment.assignee()? {
                        OrganizationRoleAssignee::OrganizationMembership(id) => {
                            entries.insert_entry(RelationshipEntry::from_parts(
                                AggregateRef::from_id::<Organization>(
                                    assignment.organization_id()?,
                                ),
                                RelationshipSubject::aggregate_set::<OrganizationMembership>(
                                    id,
                                    OrganizationMembershipUserRelation::REF,
                                ),
                            ));
                        }
                    }
                }
                Ok(entries)
            }),
        ])
    }
}
