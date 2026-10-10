use super::OrganizationMembershipUserDerivationHandlerError;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{OrganizationMembership, User};

pub struct OrganizationMembershipUserRelation;

impl Relation for OrganizationMembershipUserRelation {
    const REF: RelationRef =
        RelationRef::new(OrganizationMembership::TYPE, RelationName::new("user"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<
            OrganizationMembership,
            _,
            OrganizationMembershipUserDerivationHandlerError,
        >(|aggregate| {
            let mut entries = RelationshipEntries::new();
            if aggregate.is_active()? {
                entries.insert::<OrganizationMembership, User>(
                    aggregate.aggregate_id(),
                    *aggregate.user_id()?,
                );
            }
            Ok(entries)
        })
    }
}
