use crate::unit_of_work::UnitOfWork;

use super::relationship_eval_state::RelationshipEvalState;
use super::relationship_memo_key::RelationshipMemoKey;
use super::userset_expr_eval_context::UsersetExprEvalContext;
use super::userset_expr_eval_depth::UsersetExprEvalDepth;
use super::{
    AggregateRef, AuthorizationModel, RelationNameOwned, RelationshipRequirement,
    RelationshipResolver, RelationshipResolverConfig, RelationshipResolverError, RelationshipStore,
    RelationshipSubject, UsersetExpr,
};

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
            RelationshipRequirement::Check {
                aggregate,
                relation,
            } => {
                let relation = RelationNameOwned::from(*relation);
                self.check_relation(
                    uow,
                    subject,
                    aggregate,
                    &relation,
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
        aggregate: &AggregateRef,
        relation: &RelationNameOwned,
        state: &mut RelationshipEvalState,
        depth: UsersetExprEvalDepth,
    ) -> Result<bool, RelationshipResolverError> {
        if depth > self.config.max_depth {
            return Err(RelationshipResolverError::EvaluationLimitExceeded(
                "max_depth",
            ));
        }

        let key = RelationshipMemoKey {
            subject: subject.clone(),
            aggregate: aggregate.clone(),
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

        let Some(model) = self
            .authorization_model
            .type_definition_for(&aggregate.aggregate_type)
        else {
            state.in_progress.remove(&key);
            state.memo.insert(key, false);
            return Ok(false);
        };

        let Some(expr) = model.expr_for(&key.relation) else {
            state.in_progress.remove(&key);
            state.memo.insert(key, false);
            return Ok(false);
        };

        let context = UsersetExprEvalContext::new(subject, aggregate, &key.relation, depth);
        let result = Box::pin(self.eval_expr(uow, state, &context, expr)).await?;

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
            UsersetExpr::This => {
                let subjects = self
                    .relationship_store
                    .read_subjects_by_aggregate(uow, context.aggregate, context.relation)
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
                let relation = RelationNameOwned::from(*relation);
                Box::pin(self.check_relation(
                    uow,
                    context.subject,
                    context.aggregate,
                    &relation,
                    state,
                    context.depth.increment(),
                ))
                .await
            }
            UsersetExpr::TupleToUserset {
                tupleset_relation,
                computed_relation,
            } => {
                let tupleset_relation = RelationNameOwned::from(*tupleset_relation);
                let subjects = self
                    .relationship_store
                    .read_subjects_by_aggregate(uow, context.aggregate, &tupleset_relation)
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
                    let computed_relation = RelationNameOwned::from(*computed_relation);
                    if Box::pin(self.check_relation(
                        uow,
                        context.subject,
                        &target,
                        &computed_relation,
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
