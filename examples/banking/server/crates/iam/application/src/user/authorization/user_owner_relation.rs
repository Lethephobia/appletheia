use super::UserOwnerDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::User;

/// Allows the owning user itself.
pub struct UserOwnerRelation;

impl Relation for UserOwnerRelation {
    const REF: RelationRef = RelationRef::new(User::TYPE, RelationName::new("owner"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<User, _, UserOwnerDerivationHandlerError>(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<User, User>(aggregate.aggregate_id(), aggregate.aggregate_id());
            Ok(entries)
        })
    }
}
