use appletheia::domain::AggregateId;
use banking_ledger_domain::currency::CurrencyId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct BankingLedgerMintId([u8; 16]);

impl BankingLedgerMintId {
    pub(crate) fn into_bytes(self) -> [u8; 16] {
        self.0
    }
}

impl From<CurrencyId> for BankingLedgerMintId {
    fn from(currency_id: CurrencyId) -> Self {
        Self(*currency_id.value().as_bytes())
    }
}
