use banking_ledger_domain::core::{
    CurrencyAmount, CurrencyDecimals, SolanaMintAccountAddress, SolanaTokenAccountOwnerAddress,
};
use banking_ledger_domain::withdrawal::WithdrawalId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaWithdrawalSettlementRequest {
    withdrawal_id: WithdrawalId,
    currency_decimals: CurrencyDecimals,
    token_address: SolanaMintAccountAddress,
    token_account_owner_address: SolanaTokenAccountOwnerAddress,
    amount: CurrencyAmount,
}

impl SolanaWithdrawalSettlementRequest {
    pub fn new(
        withdrawal_id: WithdrawalId,
        currency_decimals: CurrencyDecimals,
        token_address: SolanaMintAccountAddress,
        token_account_owner_address: SolanaTokenAccountOwnerAddress,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            withdrawal_id,
            currency_decimals,
            token_address,
            token_account_owner_address,
            amount,
        }
    }

    pub fn withdrawal_id(&self) -> WithdrawalId {
        self.withdrawal_id
    }

    pub const fn currency_decimals(&self) -> CurrencyDecimals {
        self.currency_decimals
    }

    pub const fn token_address(&self) -> SolanaMintAccountAddress {
        self.token_address
    }

    pub const fn token_account_owner_address(&self) -> SolanaTokenAccountOwnerAddress {
        self.token_account_owner_address
    }

    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }
}
