use super::OrganizationFinanceManagerDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationMembership, OrganizationRole, User};

use super::OrganizationOwnerRelation;

/// Allows elevated finance managers and the owner.
pub struct OrganizationFinanceManagerRelation;

impl Relation for OrganizationFinanceManagerRelation {
    const REF: RelationRef =
        RelationRef::new(Organization::TYPE, RelationName::new("finance_manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::this::<
                OrganizationMembership,
                _,
                OrganizationFinanceManagerDerivationHandlerError,
            >(|aggregate| {
                let mut entries = RelationshipEntries::new();
                if aggregate.is_active()?
                    && aggregate
                        .roles()?
                        .iter()
                        .any(|role| *role == OrganizationRole::FinanceManager)
                {
                    entries.insert::<Organization, User>(
                        *aggregate.organization_id()?,
                        *aggregate.user_id()?,
                    );
                }
                Ok(entries)
            }),
            UsersetExpr::ComputedUserset {
                relation: OrganizationOwnerRelation::REF.into(),
            },
        ])
    }
}
