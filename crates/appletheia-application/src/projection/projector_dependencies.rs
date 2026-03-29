use super::ProjectorDescriptor;

/// Lists the projectors that an operation depends on.
///
/// These dependencies are typically used by authorization checks or command
/// handling to ensure the required projections are up to date before
/// evaluation continues.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProjectorDependencies<'a> {
    /// Indicates that no projector dependency is required.
    None,
    /// Indicates that the listed projectors must be available and up to date.
    Some(&'a [ProjectorDescriptor]),
}

impl<'a> ProjectorDependencies<'a> {
    /// Returns the dependencies as a borrowed slice.
    pub const fn as_slice(&self) -> &'a [ProjectorDescriptor] {
        match self {
            Self::None => &[],
            Self::Some(value) => value,
        }
    }

    /// Returns the dependencies as owned values.
    pub fn to_vec(&self) -> Vec<ProjectorDescriptor> {
        self.as_slice().to_vec()
    }
}
