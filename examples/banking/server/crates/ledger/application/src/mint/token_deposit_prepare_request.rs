use banking_ledger_domain::core::{CurrencyAmount, TokenAccountOwnerAddress};
use banking_ledger_domain::currency::{CurrencyId, MintAccount};

use banking_ledger_domain::deposit::DepositId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenDepositPrepareRequest {
    deposit_id: DepositId,
    currency_id: CurrencyId,
    mint_account: MintAccount,
    token_account_owner_address: TokenAccountOwnerAddress,
    amount: CurrencyAmount,
}

impl TokenDepositPrepareRequest {
    pub fn new(
        deposit_id: DepositId,
        currency_id: CurrencyId,
        mint_account: MintAccount,
        token_account_owner_address: TokenAccountOwnerAddress,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            deposit_id,
            currency_id,
            mint_account,
            token_account_owner_address,
            amount,
        }
    }

    pub fn deposit_id(&self) -> DepositId {
        self.deposit_id
    }

    pub fn currency_id(&self) -> CurrencyId {
        self.currency_id
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
}
