use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationCurrencyStatusManagerRelation;

use super::{Currency, CurrencyOwnerRelation};

/// Allows owners to manage currency status.
pub struct CurrencyStatusManagerRelation;

impl Relation for CurrencyStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("status_manager"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyOwnerRelation::REF,
            computed_userset: OrganizationCurrencyStatusManagerRelation::REF,
        },
    ]);
}
