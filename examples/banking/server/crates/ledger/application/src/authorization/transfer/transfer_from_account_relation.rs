use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::transfer::Transfer;

use super::TransferFromAccountDerivationHandlerError;

pub struct TransferFromAccountRelation;

impl Relation for TransferFromAccountRelation {
    const REF: RelationRef = RelationRef::new(Transfer::TYPE, RelationName::new("from_account"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<Transfer, _, TransferFromAccountDerivationHandlerError>(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<Transfer, Account>(
                aggregate.aggregate_id(),
                *aggregate.from_account_id()?,
            );
            Ok(entries)
        })
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::aggregate::AggregateRef;
    use appletheia::application::authorization::{
        InMemoryAuthorizationModel, Relation, RelationshipDeriver, RelationshipSubject,
    };
    use appletheia::domain::Aggregate;
    use banking_ledger_domain::account::{Account, AccountId};
    use banking_ledger_domain::core::CurrencyAmount;
    use banking_ledger_domain::transfer::{Transfer, TransferNote};

    use super::TransferFromAccountRelation;

    #[test]
    fn note_changes_preserve_the_related_account() {
        let account_id = AccountId::new();
        let mut aggregate = Transfer::new();
        aggregate
            .request(account_id, AccountId::new(), CurrencyAmount::new(1))
            .unwrap();
        let mut model = InMemoryAuthorizationModel::new();
        model.define_relation(TransferFromAccountRelation);
        let relationships = model.derive(&aggregate).unwrap();
        assert_eq!(relationships.len(), 1);
        let relationship = relationships.first().unwrap();
        assert_eq!(
            relationship.target,
            AggregateRef::from_id::<Transfer>(aggregate.aggregate_id())
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::aggregate::<Account>(account_id)
        );
        assert_eq!(
            relationship.relation,
            TransferFromAccountRelation::REF.into()
        );
        aggregate
            .set_note(Some(TransferNote::try_from("updated note").unwrap()))
            .unwrap();
        assert_eq!(model.derive(&aggregate).unwrap(), relationships);
        aggregate.set_note(None).unwrap();
        assert_eq!(model.derive(&aggregate).unwrap(), relationships);
    }
}
