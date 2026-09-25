use serde::Serialize;

use appletheia::application::read_model::ReadModelObservation;
use banking_iam_domain::{UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;

/// Identity information visible to the owning user.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UserPrivateInfoIdentity {
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
    pub observation: ReadModelObservation,
}
