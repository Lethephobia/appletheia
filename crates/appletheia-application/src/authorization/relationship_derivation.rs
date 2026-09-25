use std::{collections::HashMap, fmt, sync::Arc};

use appletheia_domain::Aggregate;

use crate::aggregate::{AggregateTypeOwned, SerializedAggregate};

use super::relationship_derivation_handler::RelationshipDerivationHandler;
use super::{RelationRefOwned, Relationship, RelationshipDerivationError, UsersetExpr};

/// Stores save-time derivations indexed by source aggregate type.
#[derive(Clone, Default)]
pub struct RelationshipDerivation {
    handlers: HashMap<AggregateTypeOwned, Vec<(RelationRefOwned, RelationshipDerivationHandler)>>,
}

impl RelationshipDerivation {
    pub(crate) fn define_relation(&mut self, relation: &RelationRefOwned, expr: &UsersetExpr) {
        for sources in self.handlers.values_mut() {
            sources.retain(|(registered, _)| registered != relation);
        }
        self.collect_sources(relation, expr);
        self.handlers.retain(|_, sources| !sources.is_empty());
        for sources in self.handlers.values_mut() {
            sources.sort_by_key(|(registered, _)| registered.to_string());
        }
    }

    fn collect_sources(&mut self, relation: &RelationRefOwned, expr: &UsersetExpr) {
        match expr {
            UsersetExpr::This(sources) => {
                for source in sources {
                    self.handlers
                        .entry(source.aggregate_type.clone())
                        .or_default()
                        .push((relation.clone(), Arc::clone(&source.handler)));
                }
            }
            UsersetExpr::Union(items) | UsersetExpr::Intersection(items) => {
                for item in items {
                    self.collect_sources(relation, item);
                }
            }
            UsersetExpr::Difference { base, subtract } => {
                self.collect_sources(relation, base);
                self.collect_sources(relation, subtract);
            }
            UsersetExpr::ComputedUserset { .. } | UsersetExpr::TupleToUserset { .. } => {}
        }
    }

    pub fn derive<A: Aggregate>(
        &self,
        aggregate: &A,
    ) -> Result<Vec<Relationship>, RelationshipDerivationError> {
        if aggregate.state().is_none() {
            return Ok(Vec::new());
        }
        let Some(handlers) = self.handlers.get(&AggregateTypeOwned::from(A::TYPE)) else {
            return Ok(Vec::new());
        };
        let serialized = SerializedAggregate::try_from_aggregate(aggregate)?;
        let mut relationships = Vec::new();
        for (relation, handler) in handlers {
            let entries = handler(&serialized).map_err(RelationshipDerivationError::Handler)?;
            for entry in entries.iter() {
                if entry.target.aggregate_type != relation.aggregate_type {
                    return Err(RelationshipDerivationError::TargetMismatch);
                }
                let relationship = Relationship {
                    target: entry.target.clone(),
                    relation: relation.clone(),
                    subject: entry.subject.clone(),
                };
                if !relationships.contains(&relationship) {
                    relationships.push(relationship);
                }
            }
        }
        Ok(relationships)
    }
}

impl fmt::Debug for RelationshipDerivation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RelationshipDerivation")
            .field("aggregate_types", &self.handlers.keys().collect::<Vec<_>>())
            .finish_non_exhaustive()
    }
}
