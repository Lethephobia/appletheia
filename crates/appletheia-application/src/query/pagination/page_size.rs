use serde::{Deserialize, Serialize};

use super::PageSizeError;

/// Maximum number of items requested from a paginated query.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub struct PageSize(u32);

impl PageSize {
    pub const MAX: u32 = 1000;

    pub fn new(value: u32) -> Result<Self, PageSizeError> {
        Self::try_from(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for PageSize {
    type Error = PageSizeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(PageSizeError::Zero);
        }

        if value > Self::MAX {
            return Err(PageSizeError::TooLarge {
                max: Self::MAX,
                actual: value,
            });
        }

        Ok(Self(value))
    }
}

impl From<PageSize> for u32 {
    fn from(value: PageSize) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero() {
        assert_eq!(PageSize::try_from(0), Err(PageSizeError::Zero));
    }

    #[test]
    fn rejects_values_above_the_limit() {
        assert_eq!(
            PageSize::try_from(PageSize::MAX + 1),
            Err(PageSizeError::TooLarge {
                max: PageSize::MAX,
                actual: PageSize::MAX + 1,
            })
        );
    }

    #[test]
    fn deserialization_preserves_validation() {
        let result = serde_json::from_value::<PageSize>(serde_json::json!(0));

        assert!(result.is_err());
    }
}
