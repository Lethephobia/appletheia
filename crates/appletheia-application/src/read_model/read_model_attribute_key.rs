use std::fmt::Display;

/// A typed attribute key. Its display value must be a valid JSON:API field name,
/// distinct from `id`, `type`, and every other field name on the read model.
pub trait ReadModelAttributeKey: Display + Copy + Eq + Send + Sync {}
