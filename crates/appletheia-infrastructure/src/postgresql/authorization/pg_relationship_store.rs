use appletheia_application::authorization::{
    Caveat, CaveatName, RebacSubject, RelationName, RelationshipEdge, RelationshipStore, ResourceRef,
};
use appletheia_application::event::{AggregateIdValue, AggregateTypeOwned};
use appletheia_application::request_context::{SubjectId, SubjectKind, SubjectRef, TenantId};
use sqlx::Row;
use uuid::Uuid;

use crate::postgresql::PgUnitOfWork;

use super::pg_relationship_store_error::PgRelationshipStoreError;

pub struct PgRelationshipStore;

impl PgRelationshipStore {
    pub fn new() -> Self {
        Self
    }
}

impl RelationshipStore<PgUnitOfWork> for PgRelationshipStore {
    type Error = PgRelationshipStoreError;

    async fn list_subjects(
        &self,
        uow: &mut PgUnitOfWork,
        tenant_id: Option<TenantId>,
        object: &ResourceRef,
        relation: &RelationName,
    ) -> Result<Vec<RelationshipEdge>, Self::Error> {
        let tenant_id = tenant_id.ok_or(PgRelationshipStoreError::InvalidRow)?;

        let transaction = uow.transaction_mut();
        let rows = sqlx::query(
            r#"
SELECT
  subject_kind,
  subject_id,
  subject_set_object_type,
  subject_set_object_id,
  subject_set_relation,
  caveat_name,
  caveat_params::text AS caveat_params_text
FROM authorization_relationship_tuples
WHERE tenant_id = $1
  AND object_type = $2
  AND object_id = $3
  AND relation = $4
"#,
        )
        .bind(tenant_id.value())
        .bind(object.aggregate_type.value())
        .bind(object.aggregate_id.value())
        .bind(relation.value())
        .fetch_all(transaction.as_mut())
        .await?;

        let mut out: Vec<RelationshipEdge> = Vec::with_capacity(rows.len());

        for row in rows {
            let subject_kind: Option<String> = row.try_get("subject_kind")?;
            let subject_id: Option<Uuid> = row.try_get("subject_id")?;
            let subject_set_object_type: Option<String> = row.try_get("subject_set_object_type")?;
            let subject_set_object_id: Option<Uuid> = row.try_get("subject_set_object_id")?;
            let subject_set_relation: Option<String> = row.try_get("subject_set_relation")?;
            let caveat_name: Option<String> = row.try_get("caveat_name")?;
            let caveat_params_text: Option<String> = row.try_get("caveat_params_text")?;

            let is_direct = subject_kind.is_some() || subject_id.is_some();
            let is_subject_set = subject_set_object_type.is_some()
                || subject_set_object_id.is_some()
                || subject_set_relation.is_some();

            let subject = match (is_direct, is_subject_set) {
                (true, false) => {
                    let kind = SubjectKind::try_from(subject_kind.ok_or(PgRelationshipStoreError::InvalidRow)?)?;
                    let id = SubjectId::from(subject_id.ok_or(PgRelationshipStoreError::InvalidRow)?);
                    RebacSubject::Subject(SubjectRef { kind, id })
                }
                (false, true) => {
                    let aggregate_type: AggregateTypeOwned =
                        subject_set_object_type.ok_or(PgRelationshipStoreError::InvalidRow)?.parse()?;
                    let aggregate_id = AggregateIdValue::from(
                        subject_set_object_id.ok_or(PgRelationshipStoreError::InvalidRow)?,
                    );
                    let relation: RelationName =
                        subject_set_relation.ok_or(PgRelationshipStoreError::InvalidRow)?.parse()?;

                    RebacSubject::SubjectSet {
                        object: ResourceRef {
                            aggregate_type,
                            aggregate_id,
                        },
                        relation,
                    }
                }
                _ => return Err(PgRelationshipStoreError::InvalidRow),
            };

            let caveat = match (caveat_name, caveat_params_text) {
                (None, None) => None,
                (Some(name), Some(params_text)) => {
                    let name: CaveatName = name.parse()?;
                    let params = serde_json::from_str(&params_text)
                        .map_err(|_| PgRelationshipStoreError::InvalidRow)?;
                    Some(Caveat { name, params })
                }
                _ => return Err(PgRelationshipStoreError::InvalidRow),
            };

            out.push(RelationshipEdge { subject, caveat });
        }

        Ok(out)
    }
}
