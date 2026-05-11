use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OidcBirthYear(u16);

impl OidcBirthYear {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn value(self) -> u16 {
        self.0
    }
}

impl Display for OidcBirthYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}", self.0)
    }
}
