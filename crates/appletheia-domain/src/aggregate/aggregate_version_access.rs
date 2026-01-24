use super::AggregateVersion;

pub trait AggregateVersionAccess {
    fn version(&self) -> AggregateVersion;

    fn set_version(&mut self, version: AggregateVersion);
}
