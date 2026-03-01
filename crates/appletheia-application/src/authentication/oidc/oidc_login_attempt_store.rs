use crate::unit_of_work::UnitOfWork;

use super::{OidcLoginAttempt, OidcLoginAttemptStoreError, OidcState};

#[allow(async_fn_in_trait)]
pub trait OidcLoginAttemptStore: Send + Sync {
    type Uow: UnitOfWork;

    async fn save(
        &self,
        uow: &mut Self::Uow,
        attempt: &OidcLoginAttempt,
    ) -> Result<(), OidcLoginAttemptStoreError>;

    async fn consume_by_state(
        &self,
        uow: &mut Self::Uow,
        state: &OidcState,
    ) -> Result<OidcLoginAttempt, OidcLoginAttemptStoreError>;
}
