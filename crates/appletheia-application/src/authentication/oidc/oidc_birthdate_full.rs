use std::fmt::{self, Display};

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use super::{OidcBirthMonth, OidcBirthYear, OidcBirthdateFullError};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OidcBirthdateFull(NaiveDate);

impl OidcBirthdateFull {
    pub fn new(
        year: OidcBirthYear,
        month: OidcBirthMonth,
        day: u8,
    ) -> Result<Self, OidcBirthdateFullError> {
        let date = NaiveDate::from_ymd_opt(year.value().into(), month.value().into(), day.into())
            .ok_or(OidcBirthdateFullError::InvalidDate)?;

        Ok(Self(date))
    }

    pub fn value(self) -> NaiveDate {
        self.0
    }

    pub fn year(self) -> OidcBirthYear {
        OidcBirthYear::new(self.0.year() as u16)
    }

    pub fn month(self) -> OidcBirthMonth {
        OidcBirthMonth::new(self.0.month() as u8).expect("NaiveDate month should be valid")
    }

    pub fn day(self) -> u8 {
        self.0.day() as u8
    }
}

impl Display for OidcBirthdateFull {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}",
            self.0.year(),
            self.0.month(),
            self.0.day()
        )
    }
}
