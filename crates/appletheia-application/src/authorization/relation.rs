use super::{RelationName, UsersetExpr};

/// Defines a statically-typed relation.
///
/// This trait is intended for in-memory and compile-time configuration of
/// authorization models, where each relation is represented by its own type.
pub trait Relation {
    /// The canonical name of this relation.
    const NAME: RelationName;

    /// Returns the userset expression that defines this relation.
    fn expr() -> UsersetExpr;
}
