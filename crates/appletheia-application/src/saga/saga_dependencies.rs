use super::SagaDescriptor;

/// Lists the sagas that an operation depends on.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SagaDependencies<'a> {
    /// Indicates that no saga dependency is required.
    None,
    /// Indicates that the listed sagas must complete before evaluation continues.
    Some(&'a [SagaDescriptor]),
}

impl<'a> SagaDependencies<'a> {
    /// Returns the dependencies as a borrowed slice.
    pub const fn as_slice(&self) -> &'a [SagaDescriptor] {
        match self {
            Self::None => &[],
            Self::Some(value) => value,
        }
    }

    /// Returns the dependencies as owned values.
    pub fn to_vec(&self) -> Vec<SagaDescriptor> {
        self.as_slice().to_vec()
    }
}
