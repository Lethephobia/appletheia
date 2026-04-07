use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationOwnerRelation;

use super::{Account, AccountOwnerRelation};

/// Allows owners to manage account status operations.
pub struct AccountStatusManagerRelation;

impl Relation for AccountStatusManagerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("status_manager"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::This,
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: AccountOwnerRelation::REF,
            computed_userset: OrganizationOwnerRelation::REF,
        },
    ]);
}
