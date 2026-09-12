use super::OrganizationAdminDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationMembership, OrganizationRole, User};

use super::OrganizationOwnerRelation;

/// Allows elevated organization administrators and the owner.
pub struct OrganizationAdminRelation;

impl Relation for OrganizationAdminRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("organization_admin"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::this::<OrganizationMembership, _, OrganizationAdminDerivationHandlerError>(
                |aggregate| {
                    let mut entries = RelationshipEntries::new();
                    if aggregate.is_active()?
                        && aggregate
                            .roles()?
                            .iter()
                            .any(|role| *role == OrganizationRole::Admin)
                    {
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
