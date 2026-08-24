use super::PreparedDepositTransaction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaDepositSettlementPreparation {
    pub transaction: PreparedDepositTransaction,
}
