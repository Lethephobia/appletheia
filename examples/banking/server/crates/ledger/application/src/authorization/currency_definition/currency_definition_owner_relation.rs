use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::CurrencyDefinition;

/// Allows the owning subject itself.
pub struct CurrencyDefinitionOwnerRelation;

impl Relation for CurrencyDefinitionOwnerRelation {
    const REF: RelationRef = RelationRef::new(CurrencyDefinition::TYPE, RelationName::new("owner"));

    const EXPR: UsersetExpr = UsersetExpr::This;
}
