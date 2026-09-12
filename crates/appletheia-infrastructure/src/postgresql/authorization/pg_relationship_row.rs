use appletheia_application::aggregate::{AggregateIdValue, AggregateRef, AggregateTypeOwned};
use appletheia_application::authorization::{
    RelationNameOwned, RelationRefOwned, Relationship, RelationshipSubject,
};
use sqlx::FromRow;
use uuid::Uuid;

use super::pg_relationship_row_error::PgRelationshipRowError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, FromRow)]
pub struct PgRelationshipRow {
    pub id: Uuid,
    pub source_aggregate_type: String,
    pub source_aggregate_id: Uuid,
    pub target_aggregate_type: String,
    pub target_aggregate_id: Uuid,
    pub relation: String,

    pub subject_aggregate_type: String,
    pub subject_aggregate_id: Option<Uuid>,
    pub subject_relation: Option<String>,
    pub subject_is_wildcard: bool,
}

impl PgRelationshipRow {
    pub fn try_into_relationship(self) -> Result<Relationship, PgRelationshipRowError> {
        let target_aggregate_type_string = self.target_aggregate_type;
        let target_aggregate_type =
            match AggregateTypeOwned::new(target_aggregate_type_string.clone()) {
                Ok(value) => value,
                Err(_) => {
                    return Err(PgRelationshipRowError::TargetAggregateType(
                        target_aggregate_type_string,
                    ));
                }
            };

        let relation_string = self.relation;
        let relation_name = match RelationNameOwned::new(relation_string.clone()) {
            Ok(value) => value,
            Err(_) => return Err(PgRelationshipRowError::Relation(relation_string)),
        };
        let relation = RelationRefOwned::new(target_aggregate_type.clone(), relation_name);

        let target = AggregateRef {
            aggregate_type: target_aggregate_type,
            aggregate_id: AggregateIdValue::from(self.target_aggregate_id),
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
                target,
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
                let subject_relation_name =
                    match RelationNameOwned::new(subject_relation_string.clone()) {
                        Ok(value) => value,
                        Err(_) => {
                            return Err(PgRelationshipRowError::SubjectRelation(
                                subject_relation_string,
                            ));
                        }
                    };
                let subject_relation = RelationRefOwned::new(
                    subject_aggregate.aggregate_type.clone(),
                    subject_relation_name,
                );
                RelationshipSubject::AggregateSet {
                    aggregate: subject_aggregate,
                    relation: subject_relation,
                }
            }
            None => RelationshipSubject::Aggregate(subject_aggregate),
        };

        Ok(Relationship {
            target,
            relation,
            subject,
        })
    }
}
