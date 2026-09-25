use appletheia::application::unit_of_work::UnitOfWork;

use super::{CurrencyList, CurrencyListReaderError};

#[allow(async_fn_in_trait)]
pub trait CurrencyListReader: Send + Sync {
    type Uow: UnitOfWork;

    async fn list(
        &self,
        uow: &mut Self::Uow,
        include_inactive: bool,
    ) -> Result<CurrencyList, CurrencyListReaderError>;
}
