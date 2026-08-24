use banking_ledger_domain::core::TokenDecimals;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EthereumTokenContractInspection {
    pub decimals: TokenDecimals,
    pub settlement_usable: bool,
}
