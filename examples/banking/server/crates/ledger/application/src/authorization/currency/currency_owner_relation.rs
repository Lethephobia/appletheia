use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::Currency;

/// Allows the owning subject itself.
pub struct CurrencyOwnerRelation;

impl Relation for CurrencyOwnerRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("owner"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
