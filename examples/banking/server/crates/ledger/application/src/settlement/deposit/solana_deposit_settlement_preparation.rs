use super::SolanaPreparedDepositTransaction;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaDepositSettlementPreparation {
    pub transaction: SolanaPreparedDepositTransaction,
}
