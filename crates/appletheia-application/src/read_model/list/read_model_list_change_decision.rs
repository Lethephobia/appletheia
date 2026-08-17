/// Describes the connection-level effect of one typed read model change.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ReadModelListChangeDecision {
    /// The change does not affect the client's materialized list range.
    Ignored,
    /// The materialized item belongs in the client's materialized list range.
    Included,
    /// List membership may have changed and must be refreshed at the client's discretion.
    Invalidated,
}
