use super::PageSizeError;

/// Maximum number of items requested from a paginated query.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PageSize(u32);

impl PageSize {
    pub const MAX: u32 = 1000;

    pub fn new(value: u32) -> Result<Self, PageSizeError> {
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

    pub fn value(&self) -> u32 {
        self.0
    }
}
