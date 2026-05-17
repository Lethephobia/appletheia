use banking_ledger_domain::currency::CurrencyMintAccount;
use serde::{Deserialize, Serialize};

use super::{MintAccountAddress, MintAccountCreateReceiptError, TokenProgramId};

/// Receipt returned after creating or retrieving an on-chain mint account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MintAccountCreateReceipt {
    address: MintAccountAddress,
    token_program_id: TokenProgramId,
}

impl MintAccountCreateReceipt {
    pub fn new(address: MintAccountAddress, token_program_id: TokenProgramId) -> Self {
        Self {
            address,
            token_program_id,
        }
    }

    pub fn address(&self) -> &MintAccountAddress {
        &self.address
    }

    pub fn token_program_id(&self) -> &TokenProgramId {
        &self.token_program_id
    }
}

impl TryFrom<MintAccountCreateReceipt> for CurrencyMintAccount {
    type Error = MintAccountCreateReceiptError;

    fn try_from(value: MintAccountCreateReceipt) -> Result<Self, Self::Error> {
        Ok(Self::new(
            value.address.try_into()?,
            value.token_program_id.try_into()?,
        ))
    }
}
