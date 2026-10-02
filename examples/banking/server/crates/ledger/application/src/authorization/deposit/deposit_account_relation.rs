use appletheia::application::authorization::{
    Relation, RelationName, RelationRef, RelationshipEntries, UsersetExpr,
};
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::Account;
use banking_ledger_domain::deposit::Deposit;

use super::DepositAccountDerivationHandlerError;

pub struct DepositAccountRelation;

impl Relation for DepositAccountRelation {
    const REF: RelationRef = RelationRef::new(Deposit::TYPE, RelationName::new("account"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::this::<Deposit, _, DepositAccountDerivationHandlerError>(|aggregate| {
            let mut entries = RelationshipEntries::new();
            entries.insert::<Deposit, Account>(aggregate.aggregate_id(), *aggregate.account_id()?);
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
    use banking_ledger_domain::core::{
        SolanaAccountAddress, SolanaTokenAccountOwnerAddress, TokenOwnerAddress,
    };
    use banking_ledger_domain::deposit::{Deposit, DepositNote};
    use banking_ledger_domain::token_binding::TokenBindingId;

    use super::DepositAccountRelation;

    #[test]
    fn note_changes_preserve_the_related_account() {
        let account_id = AccountId::new();
        let mut aggregate = Deposit::new();
        aggregate
            .request(
                account_id,
                TokenBindingId::new(),
                TokenOwnerAddress::Solana(SolanaTokenAccountOwnerAddress::new(
                    SolanaAccountAddress::from_bytes([2; 32]),
                )),
                CurrencyAmount::new(1),
            )
            .unwrap();
        let mut model = InMemoryAuthorizationModel::new();
        model.define_relation(DepositAccountRelation);
        let relationships = model.derive(&aggregate).unwrap();
        assert_eq!(relationships.len(), 1);
        let relationship = relationships.first().unwrap();
        assert_eq!(
            relationship.target,
            AggregateRef::from_id::<Deposit>(aggregate.aggregate_id())
        );
        assert_eq!(
            relationship.subject,
            RelationshipSubject::aggregate::<Account>(account_id)
        );
        assert_eq!(relationship.relation, DepositAccountRelation::REF.into());
        aggregate
            .set_note(Some(DepositNote::try_from("updated note").unwrap()))
            .unwrap();
        assert_eq!(model.derive(&aggregate).unwrap(), relationships);
        aggregate.set_note(None).unwrap();
        assert_eq!(model.derive(&aggregate).unwrap(), relationships);
    }
}
