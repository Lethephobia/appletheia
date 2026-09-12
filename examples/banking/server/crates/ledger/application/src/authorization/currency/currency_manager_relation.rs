use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency::Currency;

use super::CurrencyRegistrarRelation;
use crate::authorization::CurrencyRegistrarMemberRelation;

/// Allows members of the currency's registrar to manage it.
pub struct CurrencyManagerRelation;

impl Relation for CurrencyManagerRelation {
    const REF: RelationRef = RelationRef::new(Currency::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyRegistrarRelation::REF.into(),
            computed_userset: CurrencyRegistrarMemberRelation::REF.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{Relation, UsersetExpr};

    use super::{CurrencyManagerRelation, CurrencyRegistrarRelation};
    use crate::authorization::CurrencyRegistrarMemberRelation;

    #[test]
    fn manager_traverses_from_currency_to_registrar_members() {
        assert!(
            matches!(CurrencyManagerRelation.expr(), UsersetExpr::TupleToUserset { tupleset_relation, computed_userset } if tupleset_relation == CurrencyRegistrarRelation::REF.into() && computed_userset == CurrencyRegistrarMemberRelation::REF.into())
        );
    }
}
