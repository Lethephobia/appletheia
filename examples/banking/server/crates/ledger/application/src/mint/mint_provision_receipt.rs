use banking_ledger_domain::currency::MintAccount;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintProvisionReceipt {
    mint_account: MintAccount,
}

impl MintProvisionReceipt {
    pub fn new(mint_account: MintAccount) -> Self {
        Self { mint_account }
    }

    pub fn mint_account(&self) -> &MintAccount {
        &self.mint_account
    }

    pub fn into_mint_account(self) -> MintAccount {
        self.mint_account
    }
}
