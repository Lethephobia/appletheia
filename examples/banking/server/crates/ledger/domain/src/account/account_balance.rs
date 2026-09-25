use serde::{Deserialize, Serialize};

use crate::core::CurrencyAmount;

use super::AccountBalanceError;

/// Represents an account's total and reserved amounts.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct AccountBalance {
    total: CurrencyAmount,
    reserved: CurrencyAmount,
}

impl AccountBalance {
    /// Creates a zero account balance.
    pub fn new() -> Self {
        Self {
            total: CurrencyAmount::zero(),
            reserved: CurrencyAmount::zero(),
        }
    }

    /// Returns the total balance.
    pub const fn total(&self) -> CurrencyAmount {
        self.total
    }

    /// Returns the reserved smallest-unit quantity.
    pub const fn reserved(&self) -> CurrencyAmount {
        self.reserved
    }

    /// Returns the available balance.
    pub fn available(&self) -> Result<CurrencyAmount, AccountBalanceError> {
        Ok(self.total.try_sub(&self.reserved)?)
    }

    /// Returns a new balance with deposited amount.
    pub fn deposit(&self, amount: CurrencyAmount) -> Result<Self, AccountBalanceError> {
        Ok(Self {
            total: self.total.try_add(&amount)?,
            reserved: self.reserved,
        })
    }

    /// Returns a new balance with withdrawn amount.
    pub fn withdraw(&self, amount: CurrencyAmount) -> Result<Self, AccountBalanceError> {
        let total = self.total.try_sub(&amount)?;
        if total < self.reserved {
            return Err(AccountBalanceError::InvalidReservedBalance);
        }
        Ok(Self {
            total,
            reserved: self.reserved,
        })
    }

    /// Returns a new balance with amount reserved.
    pub fn reserve(&self, amount: CurrencyAmount) -> Result<Self, AccountBalanceError> {
        let reserved = self.reserved.try_add(&amount)?;
        if reserved > self.total {
            return Err(AccountBalanceError::InvalidReservedBalance);
        }
        Ok(Self {
            total: self.total,
            reserved,
        })
    }

    /// Returns a new balance with reserved amount released.
    pub fn release(&self, amount: CurrencyAmount) -> Result<Self, AccountBalanceError> {
        Ok(Self {
            total: self.total,
            reserved: self
                .reserved
                .try_sub(&amount)
                .map_err(|_| AccountBalanceError::InsufficientReservedBalance)?,
        })
    }

    /// Returns a new balance with reserved amount committed.
    pub fn commit(&self, amount: CurrencyAmount) -> Result<Self, AccountBalanceError> {
        let reserved = self
            .reserved
            .try_sub(&amount)
            .map_err(|_| AccountBalanceError::InsufficientReservedBalance)?;
        let total = self.total.try_sub(&amount)?;
        Ok(Self { total, reserved })
    }
}

impl Default for AccountBalance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{AccountBalance, AccountBalanceError};
    use crate::core::CurrencyAmount;

    #[test]
    fn reserve_and_commit_update_reserved_and_total_independently() {
        let balance = AccountBalance::new()
            .deposit(CurrencyAmount::new(1_000))
            .expect("deposit should succeed")
            .reserve(CurrencyAmount::new(400))
            .expect("reserve should succeed")
            .commit(CurrencyAmount::new(250))
            .expect("commit should succeed");

        assert_eq!(balance.total(), CurrencyAmount::new(750));
        assert_eq!(balance.reserved(), CurrencyAmount::new(150));
        assert_eq!(
            balance.available().expect("available should be valid"),
            CurrencyAmount::new(600)
        );
    }

    #[test]
    fn cannot_reserve_more_than_total() {
        let error = AccountBalance::new()
            .reserve(CurrencyAmount::new(1))
            .expect_err("reserve should fail");

        assert!(matches!(error, AccountBalanceError::InvalidReservedBalance));
    }
}
