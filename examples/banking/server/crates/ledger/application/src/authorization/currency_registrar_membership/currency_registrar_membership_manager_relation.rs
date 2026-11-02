use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency_registrar_membership::CurrencyRegistrarMembership;

use super::{CurrencyRegistrarMemberRelation, CurrencyRegistrarMembershipRegistrarRelation};

/// Allows registrar members to manage memberships.
pub struct CurrencyRegistrarMembershipManagerRelation;

impl Relation for CurrencyRegistrarMembershipManagerRelation {
    const REF: RelationRef = RelationRef::new(
        CurrencyRegistrarMembership::TYPE,
        RelationName::new("manager"),
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
        CurrencyRegistrarMemberRelation, CurrencyRegistrarMembershipManagerRelation,
        CurrencyRegistrarMembershipRegistrarRelation,
    };

    #[test]
    fn manager_traverses_from_membership_to_registrar_members() {
        assert!(
            matches!(CurrencyRegistrarMembershipManagerRelation.expr(), UsersetExpr::TupleToUserset { tupleset_relation, computed_userset } if tupleset_relation == CurrencyRegistrarMembershipRegistrarRelation::REF.into() && computed_userset == CurrencyRegistrarMemberRelation::REF.into())
        );
    }
}
