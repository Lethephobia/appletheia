use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Represents the OIDC `birthdate` standard claim.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum OidcBirthdate {
    Year(u16),
    YearMonth { year: u16, month: u8 },
    FullDate { year: u16, month: u8, day: u8 },
}

impl OidcBirthdate {
    /// Creates a birthdate claim with year precision.
    pub fn year(year: u16) -> Self {
        Self::Year(year)
    }

    /// Creates a birthdate claim with year-month precision.
    pub fn year_month(year: u16, month: u8) -> Result<Self, &'static str> {
        if !Self::is_valid_month(month) {
            return Err("invalid birthdate format");
        }

        Ok(Self::YearMonth { year, month })
    }

    /// Creates a birthdate claim with full date precision.
    pub fn full_date(year: u16, month: u8, day: u8) -> Result<Self, &'static str> {
        if !Self::is_valid_full_date(year, month, day) {
            return Err("invalid birthdate format");
        }

        Ok(Self::FullDate { year, month, day })
    }

    /// Returns whether the string matches a supported OIDC birthdate format.
    pub fn is_valid(value: &str) -> bool {
        Self::from_str(value).is_ok()
    }

    fn is_valid_month(month: u8) -> bool {
        (1..=12).contains(&month)
    }

    fn is_valid_full_date(year: u16, month: u8, day: u8) -> bool {
        if !Self::is_valid_month(month) {
            return false;
        }

        let max_day = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if Self::is_leap_year(year) => 29,
            2 => 28,
            _ => return false,
        };

        (1..=max_day).contains(&day)
    }

    fn is_leap_year(year: u16) -> bool {
        year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
    }
}

impl Display for OidcBirthdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Year(year) => write!(f, "{year:04}"),
            Self::YearMonth { year, month } => write!(f, "{year:04}-{month:02}"),
            Self::FullDate { year, month, day } => write!(f, "{year:04}-{month:02}-{day:02}"),
        }
    }
}

impl FromStr for OidcBirthdate {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.len() {
            4 if value.chars().all(|ch| ch.is_ascii_digit()) => {
                let year = value.parse().map_err(|_| "invalid birthdate format")?;
                Ok(Self::Year(year))
            }
            7 => {
                let (year_part, month_part) = value.split_at(4);
                if !year_part.chars().all(|ch| ch.is_ascii_digit())
                    || !month_part.starts_with('-')
                    || !month_part[1..].chars().all(|ch| ch.is_ascii_digit())
                {
                    return Err("invalid birthdate format");
                }

                let year = year_part.parse().map_err(|_| "invalid birthdate format")?;
                let month = month_part[1..]
                    .parse()
                    .map_err(|_| "invalid birthdate format")?;

                Self::year_month(year, month)
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
                    return Err("invalid birthdate format");
                }

                let year = year_part.parse().map_err(|_| "invalid birthdate format")?;
                let month = month_part.parse().map_err(|_| "invalid birthdate format")?;
                let day = day_part.parse().map_err(|_| "invalid birthdate format")?;

                Self::full_date(year, month, day)
            }
            _ => Err("invalid birthdate format"),
        }
    }
}

impl TryFrom<String> for OidcBirthdate {
    type Error = &'static str;

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
    use super::OidcBirthdate;

    #[test]
    fn accepts_full_date_birthdate() {
        let birthdate =
            OidcBirthdate::try_from("0000-10-11".to_owned()).expect("birthdate should be valid");

        assert_eq!(
            birthdate,
            OidcBirthdate::FullDate {
                year: 0,
                month: 10,
                day: 11
            }
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
                year: 2000,
                month: 1
            }
        );
        assert_eq!(birthdate.to_string(), "2000-01");
    }

    #[test]
    fn accepts_year_only_birthdate() {
        let birthdate =
            OidcBirthdate::try_from("2000".to_owned()).expect("birthdate should be valid");

        assert_eq!(birthdate, OidcBirthdate::Year(2000));
        assert_eq!(birthdate.to_string(), "2000");
    }

    #[test]
    fn rejects_invalid_birthdate() {
        let error =
            OidcBirthdate::try_from("1970/01/01".to_owned()).expect_err("birthdate should fail");

        assert_eq!(error, "invalid birthdate format");
    }

    #[test]
    fn rejects_invalid_calendar_date() {
        let error =
            OidcBirthdate::try_from("2023-02-29".to_owned()).expect_err("birthdate should fail");

        assert_eq!(error, "invalid birthdate format");
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
