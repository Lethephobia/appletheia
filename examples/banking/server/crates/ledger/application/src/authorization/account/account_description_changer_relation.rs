use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_iam_application::OrganizationFinanceManagerRelation;

use super::{Account, AccountOwnerRelation};

pub struct AccountDescriptionChangerRelation;

impl Relation for AccountDescriptionChangerRelation {
    const REF: RelationRef =
        RelationRef::new(Account::TYPE, RelationName::new("description_changer"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::Union(vec![
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
