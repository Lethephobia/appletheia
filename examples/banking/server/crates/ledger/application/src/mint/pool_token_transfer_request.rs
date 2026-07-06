use banking_ledger_domain::core::{CurrencyAmount, TokenAccountOwnerAddress};
use banking_ledger_domain::currency::{CurrencyDecimals, MintAccount};
use banking_ledger_domain::withdrawal::WithdrawalId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PoolTokenTransferRequest {
    withdrawal_id: WithdrawalId,
    mint_account: MintAccount,
    token_account_owner_address: TokenAccountOwnerAddress,
    amount: CurrencyAmount,
    decimals: CurrencyDecimals,
}

impl PoolTokenTransferRequest {
    pub fn new(
        withdrawal_id: WithdrawalId,
        mint_account: MintAccount,
        token_account_owner_address: TokenAccountOwnerAddress,
        amount: CurrencyAmount,
        decimals: CurrencyDecimals,
    ) -> Self {
        Self {
            withdrawal_id,
            mint_account,
            token_account_owner_address,
            amount,
            decimals,
        }
    }

    pub fn withdrawal_id(&self) -> WithdrawalId {
        self.withdrawal_id
    }

    pub fn mint_account(&self) -> &MintAccount {
        &self.mint_account
    }

    pub fn token_account_owner_address(&self) -> &TokenAccountOwnerAddress {
        &self.token_account_owner_address
    }

    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }

    pub fn decimals(&self) -> CurrencyDecimals {
        self.decimals
    }
}
