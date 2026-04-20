use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationOwnerRelation;

use super::{Account, AccountOwnerRelation};

/// Allows current account owners to transfer ownership.
pub struct AccountOwnershipTransfererRelation;

impl Relation for AccountOwnershipTransfererRelation {
    const REF: RelationRef =
        RelationRef::new(Account::TYPE, RelationName::new("ownership_transferer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: AccountOwnerRelation::REF,
            computed_userset: OrganizationOwnerRelation::REF,
        },
    ]);
}
