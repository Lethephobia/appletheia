use super::OrganizationOwnerDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, OrganizationOwner, User};

/// Allows the owning subject itself.
pub struct OrganizationOwnerRelation;

impl Relation for OrganizationOwnerRelation {
    const REF: RelationRef = RelationRef::new(Organization::TYPE, RelationName::new("owner"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<Organization, _, OrganizationOwnerDerivationHandlerError>(|aggregate| {
            let mut entries = RelationshipEntries::new();
            match aggregate.owner()? {
                OrganizationOwner::User(id) => {
                    entries.insert::<Organization, User>(aggregate.aggregate_id(), id)
                }
            };
            Ok(entries)
        })
    }
}
