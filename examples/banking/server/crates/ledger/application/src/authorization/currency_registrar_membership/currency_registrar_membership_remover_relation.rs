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
    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: CurrencyRegistrarMembershipRegistrarRelation::REF,
        computed_userset: CurrencyRegistrarMemberRelation::REF,
    };
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
        assert_eq!(
            CurrencyRegistrarMembershipRemoverRelation::EXPR,
            UsersetExpr::TupleToUserset {
                tupleset_relation: CurrencyRegistrarMembershipRegistrarRelation::REF,
                computed_userset: CurrencyRegistrarMemberRelation::REF,
            }
        );
    }
}
