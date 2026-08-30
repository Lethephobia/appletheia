use banking_ledger_domain::core::{
    CurrencyAmount, CurrencyDecimals, SolanaMintAccountAddress, SolanaTokenOwnerAddress,
};
use banking_ledger_domain::deposit::DepositId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaDepositSettlementPrepareRequest {
    deposit_id: DepositId,
    currency_decimals: CurrencyDecimals,
    token_address: SolanaMintAccountAddress,
    token_owner_address: SolanaTokenOwnerAddress,
    amount: CurrencyAmount,
}

impl SolanaDepositSettlementPrepareRequest {
    pub fn new(
        deposit_id: DepositId,
        currency_decimals: CurrencyDecimals,
        token_address: SolanaMintAccountAddress,
        token_owner_address: SolanaTokenOwnerAddress,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            deposit_id,
            currency_decimals,
            token_address,
            token_owner_address,
            amount,
        }
    }

    pub fn deposit_id(&self) -> DepositId {
        self.deposit_id
    }

    pub const fn currency_decimals(&self) -> CurrencyDecimals {
        self.currency_decimals
    }

    pub const fn token_address(&self) -> SolanaMintAccountAddress {
        self.token_address
    }

    pub const fn token_owner_address(&self) -> SolanaTokenOwnerAddress {
        self.token_owner_address
    }

    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }
}
