use super::{RelationRef, UsersetExpr};

/// Defines a statically-typed relation.
///
/// This trait is intended for in-memory and compile-time configuration of
/// authorization models, where each relation is represented by its own type.
pub trait Relation {
    /// The canonical reference of this relation.
    const REF: RelationRef;

    /// The userset expression of this relation.
    fn expr(&self) -> UsersetExpr;
}
