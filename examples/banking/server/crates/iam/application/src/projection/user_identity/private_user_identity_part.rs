use appletheia::application::read_model::{
    ReadModelFragment, ReadModelObservation, ReadModelObservationSource, ReadModelPart,
    ReadModelPartName,
};
use banking_iam_domain::{UserId, UserIdentityProvider, UserIdentitySubject};
use banking_shared_kernel_domain::contact::Email;
use serde::{Deserialize, Serialize};

use super::{UserIdentityFragment, UserIdentityFragmentKey};

/// Private identity information visible only to the owning user.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PrivateUserIdentityPart {
    pub user_id: UserId,
    pub provider: UserIdentityProvider,
    pub subject: UserIdentitySubject,
    pub email: Option<Email>,
    pub observation: ReadModelObservation,
}

impl From<UserIdentityFragment> for PrivateUserIdentityPart {
    fn from(fragment: UserIdentityFragment) -> Self {
        Self {
            user_id: fragment.user_id,
            provider: fragment.provider,
            subject: fragment.subject,
            email: fragment.email,
            observation: fragment.observation,
        }
    }
}

impl ReadModelObservationSource for PrivateUserIdentityPart {
    fn observations(&self) -> Vec<ReadModelObservation> {
        vec![self.observation]
    }
}

impl ReadModelPart for PrivateUserIdentityPart {
    const NAME: ReadModelPartName = ReadModelPartName::new("private_user_identity");

    type SourceFragment = UserIdentityFragment;

    fn key(&self) -> <Self::SourceFragment as ReadModelFragment>::Key {
        UserIdentityFragmentKey {
            user_id: self.user_id,
            provider: self.provider.clone(),
            subject: self.subject.clone(),
        }
    }
}
