use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationCurrencyUpdaterRelation;

use super::{Currency, CurrencyOwnerRelation};

/// Allows owners to update mutable currency attributes.
pub struct CurrencyUpdaterRelation;

impl Relation for CurrencyUpdaterRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("updater"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: CurrencyOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyOwnerRelation::REF,
            computed_userset: OrganizationCurrencyUpdaterRelation::REF,
        },
    ]);
}
