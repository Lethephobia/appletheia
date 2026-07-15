use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use super::OidcBirthMonthError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OidcBirthMonth(u8);

impl OidcBirthMonth {
    pub fn new(value: u8) -> Result<Self, OidcBirthMonthError> {
        if !(1..=12).contains(&value) {
            return Err(OidcBirthMonthError::OutOfRange { value });
        }

        Ok(Self(value))
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

impl Display for OidcBirthMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}", self.0)
    }
}
