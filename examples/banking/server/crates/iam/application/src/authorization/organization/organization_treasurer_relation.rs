use super::OrganizationTreasurerDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationMembership, OrganizationRole, User};

use super::OrganizationOwnerRelation;

/// Allows elevated treasurers and the owner.
pub struct OrganizationTreasurerRelation;

impl Relation for OrganizationTreasurerRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("treasurer"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::this::<
                OrganizationMembership,
                _,
                OrganizationTreasurerDerivationHandlerError,
            >(|aggregate| {
                let mut entries = RelationshipEntries::new();
                if aggregate.is_active()?
                    && aggregate
                        .roles()?
                        .iter()
                        .any(|role| *role == OrganizationRole::Treasurer)
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
