use super::OrganizationMemberDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationMembership, User};

use super::OrganizationOwnerRelation;

/// Allows direct members and the owning subject to count as organization members.
pub struct OrganizationMemberRelation;

impl Relation for OrganizationMemberRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("member"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::this::<OrganizationMembership, _, OrganizationMemberDerivationHandlerError>(
                |aggregate| {
                    let mut entries = RelationshipEntries::new();
                    if aggregate.is_active()? {
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
