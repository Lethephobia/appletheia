use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationCurrencyDefinitionStatusManagerRelation;

use super::{CurrencyDefinition, CurrencyDefinitionOwnerRelation};

/// Allows owners to manage currency-definition status.
pub struct CurrencyDefinitionStatusManagerRelation;

impl Relation for CurrencyDefinitionStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyDefinition::TYPE,
        RelationName::new("status_manager"),
    );

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyDefinitionOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyDefinitionOwnerRelation::REF,
            computed_userset: OrganizationCurrencyDefinitionStatusManagerRelation::REF,
        },
    ]);
}
