use super::OrganizationAdminDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationMembership, User};

use super::OrganizationOwnerRelation;

/// Allows active administrator memberships and the organization owner to administer the organization.
pub struct OrganizationAdminRelation;

impl Relation for OrganizationAdminRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("admin"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::this::<OrganizationMembership, _, OrganizationAdminDerivationHandlerError>(
                |aggregate| {
                    let mut entries = RelationshipEntries::new();
                    if aggregate.is_active()? && aggregate.is_admin()? {
                        entries.insert::<Organization, User>(
                            *aggregate.organization_id()?,
                            *aggregate.user_id()?,
                        );
                    }
                    Ok(entries)
                },
            ),
            UsersetExpr::ComputedUserset {
                relation: OrganizationOwnerRelation::REF.into(),
            },
        ])
    }
}
