use super::UnitOfWorkConfig;

pub trait UnitOfWorkConfigAccess {
    fn config(&self) -> &UnitOfWorkConfig;
}
