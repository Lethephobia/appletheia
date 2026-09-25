use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembership;

use super::{CurrencyRegistrarMemberRelation, CurrencyRegistrarMembershipRegistrarRelation};

/// Allows members of the same registrar to remove a membership.
pub struct CurrencyRegistrarMembershipRemoverRelation;

impl Relation for CurrencyRegistrarMembershipRemoverRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarMembership::TYPE,
        RelationName::new("remover"),
    );

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyRegistrarMembershipRegistrarRelation::REF.into(),
            computed_userset: CurrencyRegistrarMemberRelation::REF.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{Relation, UsersetExpr};

    use super::{
        CurrencyRegistrarMemberRelation, CurrencyRegistrarMembershipRegistrarRelation,
        CurrencyRegistrarMembershipRemoverRelation,
    };

    #[test]
    fn remover_traverses_from_membership_to_registrar_members() {
        assert!(
            matches!(CurrencyRegistrarMembershipRemoverRelation.expr(), UsersetExpr::TupleToUserset { tupleset_relation, computed_userset } if tupleset_relation == CurrencyRegistrarMembershipRegistrarRelation::REF.into() && computed_userset == CurrencyRegistrarMemberRelation::REF.into())
        );
    }
}
