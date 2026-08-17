use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource, ReadModelPartTree,
};
use serde::{Deserialize, Serialize};

mod user_public_profile_reader;
mod user_public_profile_reader_error;

pub use user_public_profile_reader::UserPublicProfileReader;
pub use user_public_profile_reader_error::UserPublicProfileReaderError;

use crate::projection::{PublicUserDetailsPart, UserFragment};

/// Public profile information visible to any caller.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserPublicProfile {
    pub user: PublicUserDetailsPart,
}

impl From<UserFragment> for UserPublicProfile {
    fn from(fragment: UserFragment) -> Self {
        Self {
            user: fragment.into(),
        }
    }
}

impl ReadModelObservationSource for UserPublicProfile {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.user.observations()
    }
}

impl ReadModel for UserPublicProfile {
    const NAME: ReadModelName = ReadModelName::new("user_public_profile");

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::field::<PublicUserDetailsPart>(
            "user",
            read_model.map(|read_model| &read_model.user),
        )]
    }
}
