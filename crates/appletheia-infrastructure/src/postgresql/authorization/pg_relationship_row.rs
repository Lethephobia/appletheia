use appletheia_application::authorization::{
    AggregateRef, RelationName, Relationship, RelationshipSubject,
};
use appletheia_application::event::{AggregateIdValue, AggregateTypeOwned};
use sqlx::FromRow;
use uuid::Uuid;

use super::pg_relationship_row_error::PgRelationshipRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgRelationshipRow {
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub relation: String,

    pub subject_aggregate_type: String,
    pub subject_aggregate_id: Option<Uuid>,
    pub subject_relation: Option<String>,
    pub subject_is_wildcard: bool,
}

impl PgRelationshipRow {
    pub fn try_into_relationship(self) -> Result<Relationship, PgRelationshipRowError> {
        let aggregate_type_string = self.aggregate_type;
        let aggregate_type = match AggregateTypeOwned::new(aggregate_type_string.clone()) {
            Ok(value) => value,
            Err(_) => return Err(PgRelationshipRowError::AggregateType(aggregate_type_string)),
        };

        let relation_string = self.relation;
        let relation = match RelationName::new(relation_string.clone()) {
            Ok(value) => value,
            Err(_) => return Err(PgRelationshipRowError::Relation(relation_string)),
        };

        let aggregate = AggregateRef {
            aggregate_type,
            aggregate_id: AggregateIdValue::from(self.aggregate_id),
        };

        let subject_aggregate_type_string = self.subject_aggregate_type;
        let subject_aggregate_type =
            match AggregateTypeOwned::new(subject_aggregate_type_string.clone()) {
                Ok(value) => value,
                Err(_) => {
                    return Err(PgRelationshipRowError::SubjectAggregateType(
                        subject_aggregate_type_string,
                    ));
                }
            };

        if self.subject_is_wildcard {
            if self.subject_aggregate_id.is_some() {
                return Err(PgRelationshipRowError::InvalidPersistedRelationship {
                    message: "wildcard subject must have NULL subject_aggregate_id",
                });
            }
            if self.subject_relation.is_some() {
                return Err(PgRelationshipRowError::InvalidPersistedRelationship {
                    message: "wildcard subject must have NULL subject_relation",
                });
            }
            return Ok(Relationship {
                aggregate,
                relation,
                subject: RelationshipSubject::Wildcard {
                    aggregate_type: subject_aggregate_type,
                },
            });
        }

        let subject_aggregate_id = self.subject_aggregate_id.ok_or(
            PgRelationshipRowError::InvalidPersistedRelationship {
                message: "non-wildcard subject must have non-NULL subject_aggregate_id",
            },
        )?;

        let subject_aggregate = AggregateRef {
            aggregate_type: subject_aggregate_type,
            aggregate_id: AggregateIdValue::from(subject_aggregate_id),
        };

        let subject = match self.subject_relation {
            Some(subject_relation_string) => {
                let subject_relation = match RelationName::new(subject_relation_string.clone()) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(PgRelationshipRowError::SubjectRelation(
                            subject_relation_string,
                        ));
                    }
                };
                RelationshipSubject::AggregateSet {
                    aggregate: subject_aggregate,
                    relation: subject_relation,
                }
            }
            None => RelationshipSubject::Aggregate(subject_aggregate),
        };

        Ok(Relationship {
            aggregate,
            relation,
            subject,
        })
    }
}
