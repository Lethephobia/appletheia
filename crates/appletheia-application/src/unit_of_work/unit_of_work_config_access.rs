use super::unit_of_work_config::UnitOfWorkConfig;

pub trait UnitOfWorkConfigAccess {
    fn config(&self) -> &UnitOfWorkConfig;
}
