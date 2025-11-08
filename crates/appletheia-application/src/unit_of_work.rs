pub mod unit_of_work_error;

pub use unit_of_work_error::UnitOfWorkError;

use appletheia_domain::Aggregate;

#[allow(async_fn_in_trait)]
pub trait UnitOfWork<A: Aggregate> {}
