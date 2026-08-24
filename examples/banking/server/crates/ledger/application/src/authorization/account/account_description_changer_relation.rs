use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationFinanceManagerRelation;

use super::{Account, AccountOwnerRelation};

pub struct AccountDescriptionChangerRelation;

impl Relation for AccountDescriptionChangerRelation {
    const REF: RelationRef =
        RelationRef::new(Account::TYPE, RelationName::new("description_changer"));

    const EXPR: UsersetExpr = UsersetExpr::Union(&[
        UsersetExpr::ComputedUserset {
            relation: AccountOwnerRelation::REF,
        },
        UsersetExpr::TupleToUserset {
            tupleset_relation: AccountOwnerRelation::REF,
            computed_userset: OrganizationFinanceManagerRelation::REF,
        },
    ]);
}
