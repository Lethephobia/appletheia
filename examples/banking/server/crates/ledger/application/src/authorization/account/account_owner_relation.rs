use appletheia::application::aggregate::AggregateRef;
use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, RelationshipEntry,
    RelationshipSubject, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_iam_application::authorization::OrganizationOwnerRelation;
use banking_iam_domain::{Organization, User};
use banking_ledger_domain::account::{Account, AccountOwner};

use super::AccountOwnerDerivationHandlerError;

/// Allows the owning user or the owner of the owning organization.
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
                    entries.insert_entry(RelationshipEntry::from_parts(
                        AggregateRef::from_id::<Account>(aggregate.aggregate_id()),
                        RelationshipSubject::aggregate_set::<Organization>(
                            id,
                            OrganizationOwnerRelation::REF,
                        ),
                    ))
                }
            };
            Ok(entries)
        })
    }
}
