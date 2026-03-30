use appletheia::relations;
use banking_ledger_domain::account::Account;

use super::{
    AccountCloserRelation, AccountDepositorRelation, AccountFreezerRelation, AccountOwnerRelation,
    AccountStatusManagerRelation, AccountThawerRelation, AccountTransferRequesterRelation,
    AccountWithdrawerRelation,
};

/// Defines static authorization relations for `Account`.
#[relations(
    aggregate = Account,
    relations = [
        AccountOwnerRelation,
        AccountStatusManagerRelation,
        AccountFreezerRelation,
        AccountThawerRelation,
        AccountCloserRelation,
        AccountDepositorRelation,
        AccountWithdrawerRelation,
        AccountTransferRequesterRelation
    ]
)]
pub struct AccountRelations;

#[cfg(test)]
mod tests {
    use appletheia::application::authorization::{
        Relation, RelationNameOwned, Relations, UsersetExpr,
    };

    use super::{
        AccountCloserRelation, AccountDepositorRelation, AccountFreezerRelation,
        AccountOwnerRelation, AccountRelations, AccountStatusManagerRelation,
        AccountThawerRelation, AccountTransferRequesterRelation, AccountWithdrawerRelation,
    };

    #[test]
    fn account_relations_define_expected_expressions() {
        let definition = AccountRelations.build();
        let owner = RelationNameOwned::from(AccountOwnerRelation::NAME);
        let status_manager = RelationNameOwned::from(AccountStatusManagerRelation::NAME);
        let freezer = RelationNameOwned::from(AccountFreezerRelation::NAME);
        let thawer = RelationNameOwned::from(AccountThawerRelation::NAME);
        let closer = RelationNameOwned::from(AccountCloserRelation::NAME);
        let depositor = RelationNameOwned::from(AccountDepositorRelation::NAME);
        let withdrawer = RelationNameOwned::from(AccountWithdrawerRelation::NAME);
        let transfer_requester = RelationNameOwned::from(AccountTransferRequesterRelation::NAME);

        assert_eq!(definition.expr_for(&owner), Some(&UsersetExpr::This));
        assert_eq!(
            definition.expr_for(&status_manager),
            Some(&UsersetExpr::This)
        );
        assert_eq!(
            definition.expr_for(&freezer),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: status_manager.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&thawer),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: status_manager,
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&closer),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: owner.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&depositor),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: owner.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&withdrawer),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset {
                    relation: owner.clone(),
                },
            ]))
        );
        assert_eq!(
            definition.expr_for(&transfer_requester),
            Some(&UsersetExpr::Union(vec![
                UsersetExpr::This,
                UsersetExpr::ComputedUserset { relation: owner },
            ]))
        );
    }
}
