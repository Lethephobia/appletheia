use crate::unit_of_work::UnitOfWork;

use super::relationship_eval_state::RelationshipEvalState;
use super::relationship_memo_key::RelationshipMemoKey;
use super::userset_expr_eval_context::UsersetExprEvalContext;
use super::userset_expr_eval_depth::UsersetExprEvalDepth;
use super::{
    AuthorizationModel, RelationRefOwned, RelationshipRequirement, RelationshipResolver,
    RelationshipResolverConfig, RelationshipResolverError, RelationshipStore, RelationshipSubject,
    UsersetExpr,
};
use crate::aggregate::AggregateRef;

#[derive(Debug)]
pub struct DefaultRelationshipResolver<RS, AM>
where
    RS: RelationshipStore,
    RS::Uow: UnitOfWork,
    AM: AuthorizationModel,
{
    relationship_store: RS,
    authorization_model: AM,
    config: RelationshipResolverConfig,
}

impl<RS, AM> DefaultRelationshipResolver<RS, AM>
where
    RS: RelationshipStore,
    RS::Uow: UnitOfWork,
    AM: AuthorizationModel,
{
    pub fn new(
        relationship_store: RS,
        authorization_model: AM,
        config: RelationshipResolverConfig,
    ) -> Self {
        Self {
            relationship_store,
            authorization_model,
            config,
        }
    }
}

impl<RS, AM> DefaultRelationshipResolver<RS, AM>
where
    RS: RelationshipStore,
    RS::Uow: UnitOfWork,
    AM: AuthorizationModel,
{
    async fn check_requirement(
        &self,
        uow: &mut RS::Uow,
        subject: &AggregateRef,
        requirement: &RelationshipRequirement,
        state: &mut RelationshipEvalState,
    ) -> Result<bool, RelationshipResolverError> {
        match requirement {
            RelationshipRequirement::Check { target, relation } => {
                self.check_relation(
                    uow,
                    subject,
                    target,
                    relation,
                    state,
                    UsersetExprEvalDepth::default(),
                )
                .await
            }
            RelationshipRequirement::All(items) => {
                for item in items {
                    if !Box::pin(self.check_requirement(uow, subject, item, state)).await? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            RelationshipRequirement::Any(items) => {
                for item in items {
                    if Box::pin(self.check_requirement(uow, subject, item, state)).await? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            RelationshipRequirement::Not(inner) => {
                Ok(!Box::pin(self.check_requirement(uow, subject, inner, state)).await?)
            }
        }
    }

    async fn check_relation(
        &self,
        uow: &mut RS::Uow,
        subject: &AggregateRef,
        target: &AggregateRef,
        relation: &RelationRefOwned,
        state: &mut RelationshipEvalState,
        depth: UsersetExprEvalDepth,
    ) -> Result<bool, RelationshipResolverError> {
        if depth > self.config.max_depth {
            return Err(RelationshipResolverError::EvaluationLimitExceeded(
                "max_depth",
            ));
        }

        if target.aggregate_type != relation.aggregate_type {
            return Err(RelationshipResolverError::InvalidRelationReference {
                target_aggregate_type: target.aggregate_type.clone(),
                relation: relation.clone(),
            });
        }

        let key = RelationshipMemoKey {
            subject: subject.clone(),
            target: target.clone(),
            relation: relation.clone(),
        };

        if let Some(&value) = state.memo.get(&key) {
            return Ok(value);
        }

        if !state.in_progress.insert(key.clone()) {
            return Ok(false);
        }

        state.node_count = state.node_count.saturating_add(1);
        if state.node_count > self.config.max_node_count {
            return Err(RelationshipResolverError::EvaluationLimitExceeded(
                "max_nodes",
            ));
        }

        let Some(expr) = self
            .authorization_model
            .expr_for(relation)
            .await
            .map_err(RelationshipResolverError::backend)?
        else {
            state.in_progress.remove(&key);
            state.memo.insert(key, false);
            return Ok(false);
        };

        let context = UsersetExprEvalContext::new(subject, target, relation, depth);
        let result = Box::pin(self.eval_expr(uow, state, &context, &expr)).await?;

        state.in_progress.remove(&key);
        state.memo.insert(key, result);
        Ok(result)
    }

    async fn eval_expr(
        &self,
        uow: &mut RS::Uow,
        state: &mut RelationshipEvalState,
        context: &UsersetExprEvalContext<'_>,
        expr: &UsersetExpr,
    ) -> Result<bool, RelationshipResolverError> {
        match expr {
            UsersetExpr::This(_) => {
                let subjects = self
                    .relationship_store
                    .read_subjects_by_target(uow, context.target, context.relation, None)
                    .await
                    .map_err(RelationshipResolverError::from)?;

                state.scanned_relationship_count = state
                    .scanned_relationship_count
                    .saturating_add(subjects.len());
                if state.scanned_relationship_count > self.config.max_scanned_relationship_count {
                    return Err(RelationshipResolverError::EvaluationLimitExceeded(
                        "max_relationships_scanned",
                    ));
                }

                for subject_ref in subjects {
                    match &subject_ref {
                        RelationshipSubject::Aggregate(target) => {
                            if target == context.subject {
                                return Ok(true);
                            }
                        }
                        RelationshipSubject::Wildcard { aggregate_type } => {
                            if aggregate_type == &context.subject.aggregate_type {
                                return Ok(true);
                            }
                        }
                        RelationshipSubject::AggregateSet {
                            aggregate: target,
                            relation: target_relation,
                        } => {
                            if Box::pin(self.check_relation(
                                uow,
                                context.subject,
                                target,
                                target_relation,
                                state,
                                context.depth.increment(),
                            ))
                            .await?
                            {
                                return Ok(true);
                            }
                        }
                    }
                }

                Ok(false)
            }
            UsersetExpr::ComputedUserset { relation } => {
                Box::pin(self.check_relation(
                    uow,
                    context.subject,
                    context.target,
                    relation,
                    state,
                    context.depth.increment(),
                ))
                .await
            }
            UsersetExpr::TupleToUserset {
                tupleset_relation,
                computed_userset,
            } => {
                let subjects = self
                    .relationship_store
                    .read_subjects_by_target(
                        uow,
                        context.target,
                        tupleset_relation,
                        Some(&computed_userset.aggregate_type),
                    )
                    .await
                    .map_err(RelationshipResolverError::from)?;

                state.scanned_relationship_count = state
                    .scanned_relationship_count
                    .saturating_add(subjects.len());
                if state.scanned_relationship_count > self.config.max_scanned_relationship_count {
                    return Err(RelationshipResolverError::EvaluationLimitExceeded(
                        "max_relationships_scanned",
                    ));
                }

                for subject_ref in subjects {
                    let RelationshipSubject::Aggregate(target) = subject_ref else {
                        continue;
                    };
                    if Box::pin(self.check_relation(
                        uow,
                        context.subject,
                        &target,
                        computed_userset,
                        state,
                        context.depth.increment(),
                    ))
                    .await?
                    {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            UsersetExpr::Union(items) => {
                for item in items {
                    if Box::pin(self.eval_expr(uow, state, context, item)).await? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            UsersetExpr::Intersection(items) => {
                for item in items {
                    if !Box::pin(self.eval_expr(uow, state, context, item)).await? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            UsersetExpr::Difference { base, subtract } => {
                let base_ok = Box::pin(self.eval_expr(uow, state, context, base)).await?;
                if !base_ok {
                    return Ok(false);
                }
                let subtract_ok = Box::pin(self.eval_expr(uow, state, context, subtract)).await?;
                Ok(!subtract_ok)
            }
        }
    }
}

impl<RS, AM> RelationshipResolver for DefaultRelationshipResolver<RS, AM>
where
    RS: RelationshipStore,
    RS::Uow: UnitOfWork,
    AM: AuthorizationModel,
{
    type Uow = RS::Uow;

    async fn satisfies(
        &self,
        uow: &mut Self::Uow,
        subject: &AggregateRef,
        requirement: &RelationshipRequirement,
    ) -> Result<bool, RelationshipResolverError> {
        let mut state = RelationshipEvalState::default();
        self.check_requirement(uow, subject, requirement, &mut state)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid::Uuid;

    use super::DefaultRelationshipResolver;
    use crate::aggregate::{AggregateIdValue, AggregateRef, AggregateTypeOwned};
    use crate::authorization::{
        InMemoryAuthorizationModel, RelationName, RelationRefOwned, Relationship,
        RelationshipRequirement, RelationshipResolver, RelationshipResolverConfig,
        RelationshipStore, RelationshipStoreError, RelationshipSubject, UsersetExpr,
    };
    use crate::unit_of_work::{UnitOfWork, UnitOfWorkError};

    #[derive(Default)]
    struct TestUow;

    impl UnitOfWork for TestUow {
        async fn commit(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }

        async fn rollback(self) -> Result<(), UnitOfWorkError> {
            Ok(())
        }
    }

    #[derive(Clone, Default)]
    struct TestStore {
        map: HashMap<(AggregateRef, RelationRefOwned), Vec<RelationshipSubject>>,
    }

    impl RelationshipStore for TestStore {
        type Uow = TestUow;

        async fn replace(
            &self,
            _uow: &mut Self::Uow,
            _source: &AggregateRef,
            _relationships: &[Relationship],
        ) -> Result<(), RelationshipStoreError> {
            Ok(())
        }

        async fn read_targets_by_subject(
            &self,
            _uow: &mut Self::Uow,
            _subject: &RelationshipSubject,
            _relation: &RelationRefOwned,
        ) -> Result<Vec<AggregateRef>, RelationshipStoreError> {
            Ok(Vec::new())
        }

        async fn read_subjects_by_target(
            &self,
            _uow: &mut Self::Uow,
            target: &AggregateRef,
            relation: &RelationRefOwned,
            subject_aggregate_type: Option<&AggregateTypeOwned>,
        ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError> {
            let subjects = self
                .map
                .get(&(target.clone(), relation.clone()))
                .cloned()
                .unwrap_or_default();

            Ok(match subject_aggregate_type {
                Some(subject_aggregate_type) => subjects
                    .into_iter()
                    .filter(|subject| match subject {
                        RelationshipSubject::Aggregate(aggregate) => {
                            &aggregate.aggregate_type == subject_aggregate_type
                        }
                        RelationshipSubject::Wildcard { aggregate_type } => {
                            aggregate_type == subject_aggregate_type
                        }
                        RelationshipSubject::AggregateSet { aggregate, .. } => {
                            &aggregate.aggregate_type == subject_aggregate_type
                        }
                    })
                    .collect(),
                None => subjects,
            })
        }
    }

    fn aggregate_type(value: &str) -> AggregateTypeOwned {
        value.parse().expect("aggregate type should be valid")
    }

    fn aggregate_ref(ty: &str, id: Uuid) -> AggregateRef {
        AggregateRef::new(aggregate_type(ty), AggregateIdValue::from(id))
    }

    fn relation_ref(aggregate_type_name: &str, relation_name: &'static str) -> RelationRefOwned {
        RelationRefOwned::new(
            aggregate_type(aggregate_type_name),
            RelationName::new(relation_name).into(),
        )
    }

    #[tokio::test]
    async fn tuple_to_userset_skips_targets_with_different_aggregate_type() {
        let document = aggregate_ref("document", Uuid::from_u128(1));
        let user = aggregate_ref("user", Uuid::from_u128(2));
        let organization = aggregate_ref("organization", Uuid::from_u128(3));

        let owner_relation = relation_ref("document", "owner");
        let status_manager_relation = relation_ref("document", "status_manager");
        let organization_owner_relation = relation_ref("organization", "owner");

        let mut store = TestStore::default();
        store.map.insert(
            (document.clone(), owner_relation.clone()),
            vec![
                RelationshipSubject::Aggregate(user.clone()),
                RelationshipSubject::Aggregate(organization.clone()),
            ],
        );
        store.map.insert(
            (organization.clone(), organization_owner_relation.clone()),
            vec![RelationshipSubject::Aggregate(user.clone())],
        );

        let mut model = InMemoryAuthorizationModel::new();
        {
            use crate::authorization::{Relation, RelationName, RelationRef};
            use appletheia_domain::AggregateType;

            struct StatusManagerRelation(UsersetExpr);

            impl Relation for StatusManagerRelation {
                const REF: RelationRef = RelationRef::new(
                    AggregateType::new("document"),
                    RelationName::new("status_manager"),
                );

                fn expr(&self) -> UsersetExpr {
                    self.0.clone()
                }
            }

            model.define_relation(StatusManagerRelation(UsersetExpr::TupleToUserset {
                tupleset_relation: owner_relation,
                computed_userset: organization_owner_relation,
            }));
        }
        {
            use crate::authorization::{Relation, RelationName, RelationRef};
            use appletheia_domain::AggregateType;

            struct OwnerRelation(UsersetExpr);

            impl Relation for OwnerRelation {
                const REF: RelationRef = RelationRef::new(
                    AggregateType::new("organization"),
                    RelationName::new("owner"),
                );

                fn expr(&self) -> UsersetExpr {
                    self.0.clone()
                }
            }

            model.define_relation(OwnerRelation(UsersetExpr::This(Vec::new())));
        }

        let resolver =
            DefaultRelationshipResolver::new(store, model, RelationshipResolverConfig::default());

        let result = resolver
            .satisfies(
                &mut TestUow,
                &user,
                &RelationshipRequirement::Check {
                    target: document,
                    relation: status_manager_relation,
                },
            )
            .await
            .expect("relationship resolution should succeed");

        assert!(result);
    }
}
