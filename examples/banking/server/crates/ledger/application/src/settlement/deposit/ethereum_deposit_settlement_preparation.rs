use super::PreparedDepositTransaction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthereumDepositSettlementPreparation {
    pub transaction: PreparedDepositTransaction,
}
