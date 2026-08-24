use banking_ledger_domain::core::{
    CurrencyAmount, CurrencyDecimals, SolanaMintAccountAddress, SolanaNetwork,
    SolanaTokenOwnerAddress, SolanaTransactionSignature,
};
use banking_ledger_domain::deposit::DepositId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaDepositSettlementVerifyRequest {
    deposit_id: DepositId,
    currency_decimals: CurrencyDecimals,
    network: SolanaNetwork,
    token_address: SolanaMintAccountAddress,
    token_owner_address: SolanaTokenOwnerAddress,
    amount: CurrencyAmount,
    transaction_id: SolanaTransactionSignature,
}

impl SolanaDepositSettlementVerifyRequest {
    pub fn new(
        deposit_id: DepositId,
        currency_decimals: CurrencyDecimals,
        network: SolanaNetwork,
        token_address: SolanaMintAccountAddress,
        token_owner_address: SolanaTokenOwnerAddress,
        amount: CurrencyAmount,
        transaction_id: SolanaTransactionSignature,
    ) -> Self {
        Self {
            deposit_id,
            currency_decimals,
            network,
            token_address,
            token_owner_address,
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

    pub const fn token_owner_address(&self) -> SolanaTokenOwnerAddress {
        self.token_owner_address
    }

    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }

    pub const fn transaction_id(&self) -> SolanaTransactionSignature {
        self.transaction_id
    }
}
