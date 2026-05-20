use super::UserId;

/// Describes the domain outcome of a user registration request.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UserRegisterResult {
    Registered { user_id: UserId },
}
