use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgRelationshipRowError {
    #[error("target_aggregate_type must be a snake_case string: {0}")]
    TargetAggregateType(String),

    #[error("relation must be a snake_case string: {0}")]
    Relation(String),

    #[error("subject_aggregate_type must be a snake_case string: {0}")]
    SubjectAggregateType(String),

    #[error("subject_relation must be a snake_case string: {0}")]
    SubjectRelation(String),

    #[error("invalid persisted relationship row: {message}")]
    InvalidPersistedRelationship { message: &'static str },
}
