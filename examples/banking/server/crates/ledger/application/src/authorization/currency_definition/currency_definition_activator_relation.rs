use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyDefinition, CurrencyDefinitionStatusManagerRelation};

/// Allows status managers to activate a currency definition.
pub struct CurrencyDefinitionActivatorRelation;

impl Relation for CurrencyDefinitionActivatorRelation {
    const REF: RelationRef =
        RelationRef::new(CurrencyDefinition::TYPE, RelationName::new("activator"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyDefinitionStatusManagerRelation::REF,
        },
    ]);
}
