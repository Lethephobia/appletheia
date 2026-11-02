use banking_ledger_domain::core::{
    CurrencyAmount, CurrencyDecimals, EvmTokenContractAddress, EvmTokenOwnerAddress,
};
use banking_ledger_domain::deposit::DepositId;

use super::EvmDepositAuthorization;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumDepositSettlementTransactionPrepareRequest {
    deposit_id: DepositId,
    currency_decimals: CurrencyDecimals,
    token_address: EvmTokenContractAddress,
    token_owner_address: EvmTokenOwnerAddress,
    amount: CurrencyAmount,
    authorization: Option<EvmDepositAuthorization>,
}

impl EthereumDepositSettlementTransactionPrepareRequest {
    pub fn new(
        deposit_id: DepositId,
        currency_decimals: CurrencyDecimals,
        token_address: EvmTokenContractAddress,
        token_owner_address: EvmTokenOwnerAddress,
        amount: CurrencyAmount,
        authorization: Option<EvmDepositAuthorization>,
    ) -> Self {
        Self {
            deposit_id,
            currency_decimals,
            token_address,
            token_owner_address,
            amount,
            authorization,
        }
    }

    pub fn deposit_id(&self) -> DepositId {
        self.deposit_id
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

    pub const fn authorization(&self) -> Option<EvmDepositAuthorization> {
        self.authorization
    }
}
