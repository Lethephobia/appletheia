use banking_ledger_domain::core::{
    CurrencyAmount, CurrencyDecimals, SolanaMintAccountAddress, SolanaNetwork,
    SolanaTransactionSignature,
};
use banking_ledger_domain::deposit::DepositId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaDepositSettlementVerifyRequest {
    deposit_id: DepositId,
    currency_decimals: CurrencyDecimals,
    network: SolanaNetwork,
    token_address: SolanaMintAccountAddress,
    amount: CurrencyAmount,
    transaction_id: SolanaTransactionSignature,
}

impl SolanaDepositSettlementVerifyRequest {
    pub fn new(
        deposit_id: DepositId,
        currency_decimals: CurrencyDecimals,
        network: SolanaNetwork,
        token_address: SolanaMintAccountAddress,
        amount: CurrencyAmount,
        transaction_id: SolanaTransactionSignature,
    ) -> Self {
        Self {
            deposit_id,
            currency_decimals,
            network,
            token_address,
            amount,
            transaction_id,
        }
    }

    pub fn deposit_id(&self) -> DepositId {
        self.deposit_id
    }

    pub const fn currency_decimals(&self) -> CurrencyDecimals {
        self.currency_decimals
    }

    pub const fn network(&self) -> SolanaNetwork {
        self.network
    }

    pub const fn token_address(&self) -> SolanaMintAccountAddress {
        self.token_address
    }

    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }

    pub const fn transaction_id(&self) -> SolanaTransactionSignature {
        self.transaction_id
    }
}
