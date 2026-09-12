use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationOwnerRelation;

use super::{Account, AccountOwnerRelation};

/// Allows current account owners to transfer ownership.
pub struct AccountOwnershipTransfererRelation;

impl Relation for AccountOwnershipTransfererRelation {
    const REF: RelationRef =
        RelationRef::new(Account::TYPE, RelationName::new("ownership_transferer"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::ComputedUserset {
                relation: AccountOwnerRelation::REF.into(),
            },
            UsersetExpr::TupleToUserset {
                tupleset_relation: AccountOwnerRelation::REF.into(),
                computed_userset: OrganizationOwnerRelation::REF.into(),
            },
        ])
    }
}
