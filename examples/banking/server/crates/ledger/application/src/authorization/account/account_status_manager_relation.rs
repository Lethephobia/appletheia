use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::Account;

/// Allows owners to manage account status operations.
pub struct AccountStatusManagerRelation;

impl Relation for AccountStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("status_manager"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
