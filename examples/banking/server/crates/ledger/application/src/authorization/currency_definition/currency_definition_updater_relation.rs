use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyDefinition, CurrencyDefinitionOwnerRelation};

/// Allows owners to update mutable currency-definition attributes.
pub struct CurrencyDefinitionUpdaterRelation;

impl Relation for CurrencyDefinitionUpdaterRelation {
    const REF: RelationRef =
        RelationRef::new(CurrencyDefinition::TYPE, RelationName::new("updater"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyDefinitionOwnerRelation::REF,
        },
    ]);
}
