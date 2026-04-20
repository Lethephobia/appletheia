use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationFinanceManagerRelation;

use super::{Currency, CurrencyOwnerRelation};

/// Allows owners to update mutable currency attributes.
pub struct CurrencyUpdaterRelation;

impl Relation for CurrencyUpdaterRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("updater"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::ComputedUserset {
            relation: CurrencyOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyOwnerRelation::REF,
            computed_userset: OrganizationFinanceManagerRelation::REF,
        },
    ]);
}
