use std::collections::{HashSet, VecDeque};

use crate::request_context::Principal;
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

use super::{
    AuthorizationPolicy, AuthorizationRequest, AuthorizationRule, Authorizer, AuthorizerError,
    CaveatEvaluator, RelationName, RelationshipEdge, RelationshipStore, ResourceRef,
};

#[derive(Debug)]
pub struct DefaultAuthorizer<U, S, P, CE> {
    uow_factory: U,
    relationship_store: S,
    policy: P,
    caveat_evaluator: CE,
    max_depth: usize,
}

impl<U, S, P, CE> DefaultAuthorizer<U, S, P, CE> {
    pub fn new(uow_factory: U, relationship_store: S, policy: P, caveat_evaluator: CE) -> Self {
        Self {
            uow_factory,
            relationship_store,
            policy,
            caveat_evaluator,
            max_depth: 8,
        }
    }

    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }
}

impl<U, S, P, CE> DefaultAuthorizer<U, S, P, CE>
where
    U: UnitOfWorkFactory,
    S: RelationshipStore<U::Uow>,
    P: AuthorizationPolicy,
    CE: CaveatEvaluator,
{
    async fn has_relation(
        &self,
        principal: &Principal,
        request: &AuthorizationRequest,
        resource: &ResourceRef,
        relation: &RelationName,
    ) -> Result<bool, AuthorizerError> {
        let (subject, tenant_id) = match principal {
            Principal::Authenticated { subject, tenant_id } => {
                (subject, tenant_id.ok_or(AuthorizerError::TenantRequired)?)
            }
            Principal::System => return Ok(true),
            Principal::Anonymous => return Err(AuthorizerError::Unauthenticated),
            Principal::Unavailable => return Err(AuthorizerError::PrincipalUnavailable),
        };

        let mut visited: HashSet<(ResourceRef, RelationName)> = HashSet::new();
        let mut queue: VecDeque<(ResourceRef, RelationName, usize)> = VecDeque::new();

        queue.push_back((resource.clone(), relation.clone(), 0));

        let mut uow = self
            .uow_factory
            .begin()
            .await
            .map_err(AuthorizerError::backend)?;

        let operation = async {
            while let Some((object, relation, depth)) = queue.pop_front() {
                if !visited.insert((object.clone(), relation.clone())) {
                    continue;
                }

                let subjects = self
                    .relationship_store
                    .list_subjects(&mut uow, Some(tenant_id), &object, &relation)
                    .await
                    .map_err(AuthorizerError::backend)?;

                for RelationshipEdge { subject: edge_subject, caveat } in subjects {
                    if let Some(caveat) = caveat.as_ref() {
                        let ok = self
                            .caveat_evaluator
                            .evaluate(principal, request, caveat)
                            .await
                            .map_err(AuthorizerError::backend)?;
                        if !ok {
                            continue;
                        }
                    }

                    match edge_subject {
                        super::RebacSubject::Subject(entry_subject) => {
                            if &entry_subject == subject {
                                return Ok(true);
                            }
                        }
                        super::RebacSubject::SubjectSet { object, relation } => {
                            if depth < self.max_depth {
                                queue.push_back((object, relation, depth + 1));
                            }
                        }
                    }
                }
            }
            Ok(false)
        };

        match operation.await {
            Ok(value) => {
                uow.commit().await.map_err(AuthorizerError::backend)?;
                Ok(value)
            }
            Err(error) => {
                let error = uow
                    .rollback_with_operation_error(error)
                    .await
                    .map_err(AuthorizerError::backend)?;
                Err(error)
            }
        }
    }
}

impl<U, S, P, CE> Authorizer for DefaultAuthorizer<U, S, P, CE>
where
    U: UnitOfWorkFactory,
    S: RelationshipStore<U::Uow>,
    P: AuthorizationPolicy,
    CE: CaveatEvaluator,
{
    async fn authorize(
        &self,
        principal: &Principal,
        request: AuthorizationRequest,
    ) -> Result<(), AuthorizerError> {
        let rule = self.policy.rule_for(request.action);

        match (&rule, principal) {
            (_, Principal::System) => return Ok(()),
            (AuthorizationRule::AllowAnonymous, _) => return Ok(()),
            (AuthorizationRule::AllowAuthenticated, Principal::Authenticated { .. }) => {
                return Ok(());
            }
            (AuthorizationRule::AllowAuthenticated, Principal::Anonymous) => {
                return Err(AuthorizerError::Unauthenticated);
            }
            (AuthorizationRule::AllowAuthenticated, Principal::Unavailable) => {
                return Err(AuthorizerError::PrincipalUnavailable);
            }
            _ => {}
        }

        let resource = request
            .resource
            .as_ref()
            .ok_or(AuthorizerError::ResourceRequired)?;

        match rule {
            AuthorizationRule::RequireRelationOnResource { relation } => {
                if self
                    .has_relation(principal, &request, resource, &relation)
                    .await?
                {
                    Ok(())
                } else {
                    Err(AuthorizerError::Forbidden)
                }
            }
            AuthorizationRule::RequireAnyRelationOnResource { relations } => {
                for relation in relations {
                    if self
                        .has_relation(principal, &request, resource, &relation)
                        .await?
                    {
                        return Ok(());
                    }
                }
                Err(AuthorizerError::Forbidden)
            }
            AuthorizationRule::AllowAnonymous | AuthorizationRule::AllowAuthenticated => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use crate::authorization::{AuthorizationAction, AuthorizationRequest};
    use crate::authorization::{Caveat, CaveatEvaluator, CaveatName};
    use crate::event::{AggregateIdValue, AggregateTypeOwned};
    use crate::request_context::{SubjectId, SubjectKind, SubjectRef, TenantId};
    use crate::unit_of_work::{UnitOfWork, UnitOfWorkError, UnitOfWorkFactory, UnitOfWorkFactoryError};

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
    struct TestUowFactory;

    impl UnitOfWorkFactory for TestUowFactory {
        type Uow = TestUow;

        async fn begin(&self) -> Result<Self::Uow, UnitOfWorkFactoryError> {
            Ok(TestUow)
        }
    }

    #[derive(Clone, Default)]
    struct TestPolicy;

    impl AuthorizationPolicy for TestPolicy {
        fn rule_for(&self, action: AuthorizationAction) -> AuthorizationRule {
            match action {
                AuthorizationAction::Command(_) => AuthorizationRule::RequireRelationOnResource {
                    relation: RelationName::try_from("editor").unwrap(),
                },
                AuthorizationAction::Query(_) => AuthorizationRule::AllowAuthenticated,
            }
        }
    }

    #[derive(Clone, Default)]
    struct TestStore {
        map: HashMap<(ResourceRef, RelationName), Vec<RelationshipEdge>>,
    }

    #[derive(Debug, thiserror::Error)]
    #[error("persistence")]
    struct TestStoreError;

    impl RelationshipStore<TestUow> for TestStore {
        type Error = TestStoreError;

        async fn list_subjects(
            &self,
            _uow: &mut TestUow,
            _tenant_id: Option<crate::request_context::TenantId>,
            object: &ResourceRef,
            relation: &RelationName,
        ) -> Result<Vec<RelationshipEdge>, Self::Error> {
            Ok(self
                .map
                .get(&(object.clone(), relation.clone()))
                .cloned()
                .unwrap_or_default())
        }
    }

    #[derive(Clone, Default)]
    struct TestCaveatEvaluator;

    #[derive(Debug, thiserror::Error)]
    #[error("caveat error")]
    struct TestCaveatError;

    impl CaveatEvaluator for TestCaveatEvaluator {
        type Error = TestCaveatError;

        async fn evaluate(
            &self,
            _principal: &Principal,
            _request: &AuthorizationRequest,
            caveat: &Caveat,
        ) -> Result<bool, Self::Error> {
            Ok(caveat.name.value() == "always_true")
        }
    }

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
    }

    #[test]
    fn allows_when_subject_has_direct_relation() {
        let rt = runtime();
        rt.block_on(async {
            let resource = ResourceRef {
                aggregate_type: AggregateTypeOwned::try_from("doc").unwrap(),
                aggregate_id: AggregateIdValue::from(uuid::Uuid::nil()),
            };
            let relation = RelationName::try_from("editor").unwrap();

            let subject = SubjectRef {
                kind: SubjectKind::try_from("user").unwrap(),
                id: SubjectId::from(uuid::Uuid::nil()),
            };

            let mut store = TestStore::default();
            store.map.insert(
                (resource.clone(), relation.clone()),
                vec![RelationshipEdge {
                    subject: super::super::RebacSubject::Subject(subject.clone()),
                    caveat: None,
                }],
            );

            let authorizer =
                DefaultAuthorizer::new(TestUowFactory, store, TestPolicy, TestCaveatEvaluator);
            let principal = Principal::Authenticated {
                subject,
                tenant_id: Some(TenantId::from(uuid::Uuid::nil())),
            };

            let result = authorizer
                .authorize(
                    &principal,
                    AuthorizationRequest {
                        action: AuthorizationAction::Command(crate::command::CommandName::new("do")),
                        resource: Some(resource),
                    },
                )
                .await;

            assert!(result.is_ok());
        });
    }

    #[test]
    fn denies_when_caveat_evaluates_to_false() {
        let rt = runtime();
        rt.block_on(async {
            let resource = ResourceRef {
                aggregate_type: AggregateTypeOwned::try_from("doc").unwrap(),
                aggregate_id: AggregateIdValue::from(uuid::Uuid::nil()),
            };
            let relation = RelationName::try_from("editor").unwrap();

            let subject = SubjectRef {
                kind: SubjectKind::try_from("user").unwrap(),
                id: SubjectId::from(uuid::Uuid::nil()),
            };

            let mut store = TestStore::default();
            store.map.insert(
                (resource.clone(), relation.clone()),
                vec![RelationshipEdge {
                    subject: super::super::RebacSubject::Subject(subject.clone()),
                    caveat: Some(Caveat {
                        name: CaveatName::try_from("always_false").unwrap(),
                        params: serde_json::Value::Null,
                    }),
                }],
            );

            let authorizer =
                DefaultAuthorizer::new(TestUowFactory, store, TestPolicy, TestCaveatEvaluator);
            let principal = Principal::Authenticated {
                subject,
                tenant_id: Some(TenantId::from(uuid::Uuid::nil())),
            };

            let result = authorizer
                .authorize(
                    &principal,
                    AuthorizationRequest {
                        action: AuthorizationAction::Command(crate::command::CommandName::new(
                            "do",
                        )),
                        resource: Some(resource),
                    },
                )
                .await;

            assert!(matches!(result, Err(AuthorizerError::Forbidden)));
        });
    }
}
