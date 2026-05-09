/// Lifecycle status tracked by public user profile projections.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserPublicProfileStatus {
    Active,
    Inactive,
}
