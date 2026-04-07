use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::Account;

/// Allows the owning user itself.
pub struct AccountOwnerRelation;

impl Relation for AccountOwnerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("owner"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
