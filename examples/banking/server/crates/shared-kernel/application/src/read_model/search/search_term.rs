use std::fmt::{self, Display};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::SearchTermError;

/// Represents a non-empty, normalized term used for text search.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct SearchTerm(String);

impl TryFrom<&str> for SearchTerm {
    type Error = SearchTermError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

impl TryFrom<String> for SearchTerm {
    type Error = SearchTermError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl SearchTerm {
    /// Creates a search term by removing whitespace and folding characters to lowercase.
    pub fn new(value: String) -> Result<Self, SearchTermError> {
        let normalized_value = Self::normalize(&value);
        if normalized_value.is_empty() {
            return Err(SearchTermError::Empty);
        }

        Ok(Self(normalized_value))
    }

    /// Returns whether this term occurs after applying the same normalization to `candidate`.
    pub fn matches(&self, candidate: impl AsRef<str>) -> bool {
        let normalized_candidate = Self::normalize(candidate.as_ref());
        normalized_candidate.contains(&self.0)
    }

    fn normalize(value: &str) -> String {
        value
            .chars()
            .filter(|character| !character.is_whitespace())
            .flat_map(char::to_lowercase)
            .collect()
    }
}

impl AsRef<str> for SearchTerm {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for SearchTerm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_ref())
    }
}

impl FromStr for SearchTerm {
    type Err = SearchTermError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl From<SearchTerm> for String {
    fn from(value: SearchTerm) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_case_and_whitespace_when_constructed() {
        let term = SearchTerm::try_from("  ALI\u{3000}CE\u{00a0}_1 ")
            .expect("search term should be valid");

        assert_eq!(term.as_ref(), "alice_1");
    }

    #[test]
    fn rejects_a_term_containing_only_whitespace() {
        let error = SearchTerm::try_from(" \u{3000}\u{00a0}")
            .expect_err("empty normalized term should fail");

        assert_eq!(error, SearchTermError::Empty);
    }

    #[test]
    fn matches_a_candidate_using_the_same_normalization() {
        let term = SearchTerm::try_from(" ALI ").expect("search term should be valid");

        assert!(term.matches("  Alice Example "));
        assert!(!term.matches("Bob Example"));
    }
}
