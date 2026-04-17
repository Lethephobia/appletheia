use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationCurrencyIssuerRelation;

use super::{Currency, CurrencyOwnerRelation};

/// Allows owners to issue supply for a currency.
pub struct CurrencyIssuerRelation;

impl Relation for CurrencyIssuerRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("issuer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyOwnerRelation::REF,
            computed_userset: OrganizationCurrencyIssuerRelation::REF,
        },
    ]);
}
