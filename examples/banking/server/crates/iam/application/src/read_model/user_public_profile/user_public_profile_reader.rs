use appletheia::application::unit_of_work::UnitOfWork;
use banking_iam_domain::UserId;

use super::{UserPublicProfile, UserPublicProfileReaderError};

/// Loads public user profile read models from query-side tables.
#[allow(async_fn_in_trait)]
pub trait UserPublicProfileReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn find(
        &self,
        uow: &mut Self::Uow,
        user_id: UserId,
    ) -> Result<Option<UserPublicProfile>, UserPublicProfileReaderError>;
}
