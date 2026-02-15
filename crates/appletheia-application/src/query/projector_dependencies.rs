use crate::projection::ProjectorName;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProjectorDependencies<'a> {
    None,
    Some(&'a [ProjectorName]),
}

impl<'a> ProjectorDependencies<'a> {
    pub const fn as_slice(&self) -> &'a [ProjectorName] {
        match self {
            Self::None => &[],
            Self::Some(value) => value,
        }
    }
}
