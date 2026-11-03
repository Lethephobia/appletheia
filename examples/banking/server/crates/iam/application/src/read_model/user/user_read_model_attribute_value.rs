use appletheia::read_model_attribute_value;
use banking_iam_domain::{UserBio, UserDisplayName, Username};

#[read_model_attribute_value]
pub enum UserReadModelAttributeValue {
    Username(Option<Username>),
    DisplayName(Option<UserDisplayName>),
    Bio(Option<UserBio>),
}
