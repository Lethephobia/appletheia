use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationOwnerRelation;

use super::{CurrencyDefinition, CurrencyDefinitionOwnerRelation};

/// Allows owners to issue supply for a currency definition.
pub struct CurrencyDefinitionIssuerRelation;

impl Relation for CurrencyDefinitionIssuerRelation {
    const REF: RelationRef =
        RelationRef::new(CurrencyDefinition::TYPE, RelationName::new("issuer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyDefinitionOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyDefinitionOwnerRelation::REF,
            computed_userset: OrganizationOwnerRelation::REF,
        },
    ]);
}
