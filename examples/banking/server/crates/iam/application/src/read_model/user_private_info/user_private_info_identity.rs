use banking_iam_domain::{UserIdentityProvider, UserIdentitySubject, core::Email};

use crate::read_model::ReadModelObservation;

/// Identity information visible to the owning user.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPrivateInfoIdentity {
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
    pub observation: ReadModelObservation,
}
