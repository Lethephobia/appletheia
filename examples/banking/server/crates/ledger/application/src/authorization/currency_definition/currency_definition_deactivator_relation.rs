use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;

use super::{CurrencyDefinition, CurrencyDefinitionStatusManagerRelation};

/// Allows status managers to deactivate a currency definition.
pub struct CurrencyDefinitionDeactivatorRelation;

impl Relation for CurrencyDefinitionDeactivatorRelation {
    const REF: RelationRef =
        RelationRef::new(CurrencyDefinition::TYPE, RelationName::new("deactivator"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyDefinitionStatusManagerRelation::REF,
        },
    ]);
}
