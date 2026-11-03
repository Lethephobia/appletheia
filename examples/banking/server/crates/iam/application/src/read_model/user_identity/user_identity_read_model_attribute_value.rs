use appletheia::read_model_attribute_value;
use banking_iam_domain::{UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;

#[read_model_attribute_value]
pub enum UserIdentityReadModelAttributeValue {
    Provider(UserIdentityProvider),
    Subject(UserIdentitySubject),
    Email(Option<Email>),
}
