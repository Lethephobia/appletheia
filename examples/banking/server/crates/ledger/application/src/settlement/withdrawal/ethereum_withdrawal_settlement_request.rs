use banking_ledger_domain::core::{
    CurrencyAmount, CurrencyDecimals, EvmTokenContractAddress, EvmTokenOwnerAddress,
};
use banking_ledger_domain::withdrawal::WithdrawalId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumWithdrawalSettlementRequest {
    withdrawal_id: WithdrawalId,
    currency_decimals: CurrencyDecimals,
    token_address: EvmTokenContractAddress,
    token_owner_address: EvmTokenOwnerAddress,
    amount: CurrencyAmount,
}

impl EthereumWithdrawalSettlementRequest {
    pub fn new(
        withdrawal_id: WithdrawalId,
        currency_decimals: CurrencyDecimals,
        token_address: EvmTokenContractAddress,
        token_owner_address: EvmTokenOwnerAddress,
        amount: CurrencyAmount,
    ) -> Self {
        Self {
            withdrawal_id,
            currency_decimals,
            token_address,
            token_owner_address,
            amount,
        }
    }

    pub fn withdrawal_id(&self) -> WithdrawalId {
        self.withdrawal_id
    }

    pub const fn currency_decimals(&self) -> CurrencyDecimals {
        self.currency_decimals
    }

    pub const fn token_address(&self) -> EvmTokenContractAddress {
        self.token_address
    }

    pub const fn token_owner_address(&self) -> EvmTokenOwnerAddress {
        self.token_owner_address
    }

    pub fn amount(&self) -> CurrencyAmount {
        self.amount
    }
}
