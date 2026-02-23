use crate::unit_of_work::UnitOfWork;

use super::relationship_eval_state::RelationshipEvalState;
use super::relationship_memo_key::RelationshipMemoKey;
use super::{
    AggregateRef, AuthorizationModel, AuthorizationTypeDefinition, RelationName,
    RelationshipRequirement, RelationshipResolver, RelationshipResolverConfig,
    RelationshipResolverError, RelationshipStore, RelationshipSubject, UsersetExpr,
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
            RelationshipRequirement::None => Ok(true),
            RelationshipRequirement::Check {
                aggregate,
                relation,
            } => {
                self.check_relation(uow, subject, aggregate, relation, state, 0)
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
        relation: &RelationName,
        state: &mut RelationshipEvalState,
        depth: usize,
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

        if let Some(value) = state.memo.get(&key) {
            return Ok(*value);
        }

        if !state.in_progress.insert(key.clone()) {
            return Ok(false);
        }

        state.nodes = state.nodes.saturating_add(1);
        if state.nodes > self.config.max_nodes {
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

        let Some(expr) = model.expr_for(relation) else {
            state.in_progress.remove(&key);
            state.memo.insert(key, false);
            return Ok(false);
        };

        let result =
            Box::pin(self.eval_expr(uow, subject, aggregate, relation, model, expr, state, depth))
                .await?;

        state.in_progress.remove(&key);
        state.memo.insert(key, result);
        Ok(result)
    }

    async fn eval_expr(
        &self,
        uow: &mut RS::Uow,
        subject: &AggregateRef,
        aggregate: &AggregateRef,
        current_relation: &RelationName,
        model: &AuthorizationTypeDefinition,
        expr: &UsersetExpr,
        state: &mut RelationshipEvalState,
        depth: usize,
    ) -> Result<bool, RelationshipResolverError> {
        match expr {
            UsersetExpr::This => {
                let subjects = self
                    .relationship_store
                    .read_subjects_by_aggregate(uow, aggregate, current_relation)
                    .await
                    .map_err(RelationshipResolverError::from)?;

                state.relationships_scanned =
                    state.relationships_scanned.saturating_add(subjects.len());
                if state.relationships_scanned > self.config.max_relationships_scanned {
                    return Err(RelationshipResolverError::EvaluationLimitExceeded(
                        "max_relationships_scanned",
                    ));
                }

                for subject_ref in subjects {
                    match &subject_ref {
                        RelationshipSubject::Aggregate(target) => {
                            if target == subject {
                                return Ok(true);
                            }
                        }
                        RelationshipSubject::Wildcard { aggregate_type } => {
                            if aggregate_type == &subject.aggregate_type {
                                return Ok(true);
                            }
                        }
                        RelationshipSubject::AggregateSet {
                            aggregate: target,
                            relation: target_relation,
                        } => {
                            if Box::pin(self.check_relation(
                                uow,
                                subject,
                                target,
                                target_relation,
                                state,
                                depth + 1,
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
                Box::pin(self.check_relation(uow, subject, aggregate, relation, state, depth + 1))
                    .await
            }
            UsersetExpr::TupleToUserset {
                tupleset_relation,
                computed_relation,
            } => {
                let subjects = self
                    .relationship_store
                    .read_subjects_by_aggregate(uow, aggregate, tupleset_relation)
                    .await
                    .map_err(RelationshipResolverError::from)?;

                state.relationships_scanned =
                    state.relationships_scanned.saturating_add(subjects.len());
                if state.relationships_scanned > self.config.max_relationships_scanned {
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
                        subject,
                        &target,
                        computed_relation,
                        state,
                        depth + 1,
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
                    if Box::pin(self.eval_expr(
                        uow,
                        subject,
                        aggregate,
                        current_relation,
                        model,
                        item,
                        state,
                        depth,
                    ))
                    .await?
                    {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            UsersetExpr::Intersection(items) => {
                for item in items {
                    if !Box::pin(self.eval_expr(
                        uow,
                        subject,
                        aggregate,
                        current_relation,
                        model,
                        item,
                        state,
                        depth,
                    ))
                    .await?
                    {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            UsersetExpr::Difference { base, subtract } => {
                let base_ok = Box::pin(self.eval_expr(
                    uow,
                    subject,
                    aggregate,
                    current_relation,
                    model,
                    base,
                    state,
                    depth,
                ))
                .await?;
                if !base_ok {
                    return Ok(false);
                }
                let subtract_ok = Box::pin(self.eval_expr(
                    uow,
                    subject,
                    aggregate,
                    current_relation,
                    model,
                    subtract,
                    state,
                    depth,
                ))
                .await?;
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
