use std::fmt::{self, Display};

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

use super::OidcBirthdateFullError;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct OidcBirthdateFull(NaiveDate);

impl OidcBirthdateFull {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, OidcBirthdateFullError> {
        let date = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
            .ok_or(OidcBirthdateFullError::InvalidDate)?;

        Ok(Self(date))
    }

    pub fn value(self) -> NaiveDate {
        self.0
    }

    pub fn year(self) -> u16 {
        self.0.year() as u16
    }

    pub fn month(self) -> u8 {
        self.0.month() as u8
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_numeric_date_components_together() {
        for (year, month, day) in [
            (2023, 2, 29),
            (2024, 0, 1),
            (2024, 13, 1),
            (2024, 1, 0),
            (2024, 4, 31),
        ] {
            assert!(matches!(
                OidcBirthdateFull::new(year, month, day),
                Err(OidcBirthdateFullError::InvalidDate)
            ));
        }
        let date = OidcBirthdateFull::new(2024, 2, 29).unwrap();
        assert_eq!((date.year(), date.month(), date.day()), (2024, 2, 29));
    }
}
