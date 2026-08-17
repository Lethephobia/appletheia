use banking_ledger_domain::account::AccountId;
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::CurrencyId;
use banking_ledger_domain::currency_issuance::CurrencyIssuanceId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountTransactionCurrencyIssuanceIssuedRecord {
    pub id: CurrencyIssuanceId,
    pub destination_account_id: AccountId,
    pub currency_id: CurrencyId,
    pub amount: CurrencyAmount,
}
