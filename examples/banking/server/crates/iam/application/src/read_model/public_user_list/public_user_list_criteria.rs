/// Search criteria for public user list reads.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PublicUserListCriteria {
    pub username_contains: Vec<String>,
}
