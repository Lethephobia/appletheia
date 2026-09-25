use super::AccountOwnerDerivationHandlerError;

use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::account::{Account, AccountOwner};

/// Allows the owning subject itself.
pub struct AccountOwnerRelation;

impl Relation for AccountOwnerRelation {
    const REF: RelationRef = RelationRef::new(Account::TYPE, RelationName::new("owner"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<Account, _, AccountOwnerDerivationHandlerError>(|aggregate| {
            let mut entries = RelationshipEntries::new();
            match aggregate.owner()? {
                AccountOwner::User(id) => {
                    entries.insert::<Account, User>(aggregate.aggregate_id(), id)
                }
                AccountOwner::Organization(id) => {
                    entries.insert::<Account, Organization>(aggregate.aggregate_id(), id)
                }
            };
            Ok(entries)
        })
    }
}
