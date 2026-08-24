use super::EvmPreparedDepositTransaction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumDepositSettlementPreparation {
    pub transaction: EvmPreparedDepositTransaction,
}
