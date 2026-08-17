use appletheia::application::read_model::{
    ReadModelFragment, ReadModelFragmentName, ReadModelObservation, ReadModelObservationSource,
};
use appletheia::domain::EventOccurredAt;
use banking_iam_domain::{UserId, UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;
use serde::{Deserialize, Serialize};

use super::UserIdentityFragmentKey;

/// Complete user identity fragment shared by read models.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserIdentityFragment {
    pub user_id: UserId,
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
    pub created_at: EventOccurredAt,
    pub observation: ReadModelObservation,
}

impl ReadModelObservationSource for UserIdentityFragment {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelFragment for UserIdentityFragment {
    const NAME: ReadModelFragmentName = ReadModelFragmentName::new("user_identity_fragment");

    type Key = UserIdentityFragmentKey;

    fn key(&self) -> Self::Key {
        UserIdentityFragmentKey {
            user_id: self.user_id,
            provider: self.provider.clone(),
            subject: self.subject.clone(),
        }
    }
}
