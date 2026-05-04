use super::PageLimitError;

/// Maximum number of items requested from a paginated query.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PageLimit(u32);

impl PageLimit {
    pub const MAX: u32 = 1000;

    pub fn new(value: u32) -> Result<Self, PageLimitError> {
        if value == 0 {
            return Err(PageLimitError::Zero);
        }

        if value > Self::MAX {
            return Err(PageLimitError::TooLarge {
                max: Self::MAX,
                actual: value,
            });
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}
