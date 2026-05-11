use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{
    OidcBirthMonth, OidcBirthYear, OidcBirthYearError, OidcBirthdateError, OidcBirthdateFull,
};

/// Represents the OIDC `birthdate` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum OidcBirthdate {
    Year(OidcBirthYear),
    YearMonth {
        year: OidcBirthYear,
        month: OidcBirthMonth,
    },
    Full(OidcBirthdateFull),
}

impl OidcBirthdate {
    /// Creates a birthdate claim with year precision.
    pub fn year(year: u16) -> Self {
        Self::Year(OidcBirthYear::new(year))
    }

    /// Creates a birthdate claim with year-month precision.
    pub fn year_month(year: u16, month: u8) -> Result<Self, OidcBirthdateError> {
        let year = OidcBirthYear::new(year);
        let month = OidcBirthMonth::new(month)?;

        Ok(Self::YearMonth { year, month })
    }

    /// Creates a birthdate claim with full date precision.
    pub fn full(year: u16, month: u8, day: u8) -> Result<Self, OidcBirthdateError> {
        let year = OidcBirthYear::new(year);
        let month = OidcBirthMonth::new(month)?;
        let full = OidcBirthdateFull::new(year, month, day)?;

        Ok(Self::Full(full))
    }

    /// Returns whether the string matches a supported OIDC birthdate format.
    pub fn is_valid(value: &str) -> bool {
        Self::from_str(value).is_ok()
    }

    fn parse_year(value: &str) -> Result<OidcBirthYear, OidcBirthdateError> {
        let value = value
            .parse()
            .map_err(|_| OidcBirthYearError::InvalidFormat)?;

        Ok(OidcBirthYear::new(value))
    }

    fn parse_month(value: &str) -> Result<OidcBirthMonth, OidcBirthdateError> {
        let value = value
            .parse()
            .map_err(|_| OidcBirthdateError::InvalidFormat)?;

        Ok(OidcBirthMonth::new(value)?)
    }
}

impl Display for OidcBirthdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Year(year) => write!(f, "{year}"),
            Self::YearMonth { year, month } => write!(f, "{year}-{month}"),
            Self::Full(full) => write!(f, "{full}"),
        }
    }
}

impl FromStr for OidcBirthdate {
    type Err = OidcBirthdateError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.len() {
            4 if value.chars().all(|ch| ch.is_ascii_digit()) => {
                Ok(Self::Year(Self::parse_year(value)?))
            }
            7 => {
                let (year_part, month_part) = value.split_at(4);
                if !year_part.chars().all(|ch| ch.is_ascii_digit())
                    || !month_part.starts_with('-')
                    || !month_part[1..].chars().all(|ch| ch.is_ascii_digit())
                {
                    return Err(OidcBirthdateError::InvalidFormat);
                }

                let year = Self::parse_year(year_part)?;
                let month = Self::parse_month(&month_part[1..])?;

                Ok(Self::YearMonth { year, month })
            }
            10 => {
                let year_part = &value[..4];
                let month_part = &value[5..7];
                let day_part = &value[8..10];

                if value.as_bytes()[4] != b'-'
                    || value.as_bytes()[7] != b'-'
                    || !year_part.chars().all(|ch| ch.is_ascii_digit())
                    || !month_part.chars().all(|ch| ch.is_ascii_digit())
                    || !day_part.chars().all(|ch| ch.is_ascii_digit())
                {
                    return Err(OidcBirthdateError::InvalidFormat);
                }

                let year = Self::parse_year(year_part)?;
                let month = Self::parse_month(month_part)?;
                let day = day_part
                    .parse()
                    .map_err(|_| OidcBirthdateError::InvalidFormat)?;
                let full = OidcBirthdateFull::new(year, month, day)?;

                Ok(Self::Full(full))
            }
            _ => Err(OidcBirthdateError::InvalidFormat),
        }
    }
}

impl TryFrom<String> for OidcBirthdate {
    type Error = OidcBirthdateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl From<OidcBirthdate> for String {
    fn from(value: OidcBirthdate) -> Self {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        OidcBirthMonth, OidcBirthYear, OidcBirthdate, OidcBirthdateError, OidcBirthdateFull,
    };
    use crate::authentication::oidc::OidcBirthdateFullError;

    #[test]
    fn accepts_full_birthdate() {
        let birthdate =
            OidcBirthdate::try_from("0000-10-11".to_owned()).expect("birthdate should be valid");

        assert_eq!(
            birthdate,
            OidcBirthdate::Full(
                OidcBirthdateFull::new(OidcBirthYear::new(0), OidcBirthMonth::new(10).unwrap(), 11)
                    .unwrap()
            )
        );
        assert_eq!(birthdate.to_string(), "0000-10-11");
    }

    #[test]
    fn accepts_year_month_birthdate() {
        let birthdate =
            OidcBirthdate::try_from("2000-01".to_owned()).expect("birthdate should be valid");

        assert_eq!(
            birthdate,
            OidcBirthdate::YearMonth {
                year: OidcBirthYear::new(2000),
                month: OidcBirthMonth::new(1).unwrap()
            }
        );
        assert_eq!(birthdate.to_string(), "2000-01");
    }

    #[test]
    fn accepts_year_only_birthdate() {
        let birthdate =
            OidcBirthdate::try_from("2000".to_owned()).expect("birthdate should be valid");

        assert_eq!(birthdate, OidcBirthdate::Year(OidcBirthYear::new(2000)));
        assert_eq!(birthdate.to_string(), "2000");
    }

    #[test]
    fn rejects_invalid_birthdate() {
        let error =
            OidcBirthdate::try_from("1970/01/01".to_owned()).expect_err("birthdate should fail");

        assert_eq!(error, OidcBirthdateError::InvalidFormat);
    }

    #[test]
    fn rejects_invalid_calendar_date() {
        let error =
            OidcBirthdate::try_from("2023-02-29".to_owned()).expect_err("birthdate should fail");

        assert_eq!(
            error,
            OidcBirthdateError::Full(OidcBirthdateFullError::InvalidDate)
        );
    }

    #[test]
    fn is_valid_returns_true_for_supported_format() {
        assert!(OidcBirthdate::is_valid("2000-01-31"));
        assert!(OidcBirthdate::is_valid("2000-01"));
        assert!(OidcBirthdate::is_valid("2000"));
    }

    #[test]
    fn is_valid_returns_false_for_unsupported_format() {
        assert!(!OidcBirthdate::is_valid("2000/01/31"));
        assert!(!OidcBirthdate::is_valid("2000-13"));
    }
}
