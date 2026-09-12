use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationFinanceManagerRelation;

use super::{Account, AccountOwnerRelation};

/// Allows owners to manage account status operations.
pub struct AccountStatusManagerRelation;

impl Relation for AccountStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("status_manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
            UsersetExpr::This(Vec::new()),
            UsersetExpr::ComputedUserset {
                relation: AccountOwnerRelation::REF.into(),
            },
            UsersetExpr::TupleToUserset {
                tupleset_relation: AccountOwnerRelation::REF.into(),
                computed_userset: OrganizationFinanceManagerRelation::REF.into(),
            },
        ])
    }
}
