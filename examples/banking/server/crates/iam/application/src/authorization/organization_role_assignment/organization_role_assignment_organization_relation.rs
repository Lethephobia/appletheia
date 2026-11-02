use super::OrganizationRoleAssignmentOrganizationDerivationHandlerError;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::Organization;
use banking_iam_domain::OrganizationRoleAssignment;

pub struct OrganizationRoleAssignmentOrganizationRelation;

impl Relation for OrganizationRoleAssignmentOrganizationRelation {
    const REF: RelationRef = RelationRef::new(
        OrganizationRoleAssignment::TYPE,
        RelationName::new("organization"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            OrganizationRoleAssignment,
            _,
            OrganizationRoleAssignmentOrganizationDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<OrganizationRoleAssignment, Organization>(
                aggregate.aggregate_id(),
                aggregate.organization_id()?,
            );
            Ok(entries)
        })
    }
}
