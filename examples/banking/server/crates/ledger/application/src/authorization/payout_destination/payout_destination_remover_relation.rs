use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationFinanceManagerRelation;

use super::{PayoutDestination, PayoutDestinationOwnerRelation};

/// Allows owners to remove a payout destination.
pub struct PayoutDestinationRemoverRelation;

impl Relation for PayoutDestinationRemoverRelation {
    const REF: RelationRef =
        RelationRef::new(PayoutDestination::TYPE, RelationName::new("remover"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::ComputedUserset {
            relation: PayoutDestinationOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: PayoutDestinationOwnerRelation::REF,
            computed_userset: OrganizationFinanceManagerRelation::REF,
        },
    ]);
}
