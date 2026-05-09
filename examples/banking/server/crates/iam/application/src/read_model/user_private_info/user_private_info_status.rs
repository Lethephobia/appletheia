/// Lifecycle status visible in user-private information.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UserPrivateInfoStatus {
    Active,
    Inactive,
}
