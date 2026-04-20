use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationOwnerRelation;

use super::{Currency, CurrencyOwnerRelation};

/// Allows current currency owners to transfer ownership.
pub struct CurrencyOwnershipTransfererRelation;

impl Relation for CurrencyOwnershipTransfererRelation {
    const REF: RelationRef =
        RelationRef::new(Currency::TYPE, RelationName::new("ownership_transferer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::ComputedUserset {
            relation: CurrencyOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: CurrencyOwnerRelation::REF,
            computed_userset: OrganizationOwnerRelation::REF,
        },
    ]);
}
