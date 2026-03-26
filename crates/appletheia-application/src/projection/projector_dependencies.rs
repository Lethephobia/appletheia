use super::{ProjectorName, ProjectorNameOwned};

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
    Some(&'a [ProjectorName]),
}

impl<'a> ProjectorDependencies<'a> {
    /// Returns the dependencies as a borrowed slice.
    pub const fn as_slice(&self) -> &'a [ProjectorName] {
        match self {
            Self::None => &[],
            Self::Some(value) => value,
        }
    }

    /// Returns owned projector names for persistence or runtime lookup.
    pub fn owned_names(&self) -> Vec<ProjectorNameOwned> {
        self.as_slice()
            .iter()
            .copied()
            .map(ProjectorNameOwned::from)
            .collect()
    }
}
