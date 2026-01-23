use serde::{Deserialize, Serialize};
use std::{convert::Infallible, fmt, fmt::Display, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AggregateTypeOwned(String);

impl AggregateTypeOwned {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for AggregateTypeOwned {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl From<String> for AggregateTypeOwned {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for AggregateTypeOwned {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl Display for AggregateTypeOwned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_value() {
        let agg: AggregateTypeOwned = "user_profile".parse().unwrap();
        assert_eq!(agg.to_string(), "user_profile");
    }
}
