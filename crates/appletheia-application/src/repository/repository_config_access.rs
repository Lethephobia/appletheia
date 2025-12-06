use super::RepositoryConfig;

pub trait RepositoryConfigAccess {
    fn config(&self) -> &RepositoryConfig;
}
