use crate::request_context::Principal;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{
    AuthorizationPlan, Authorizer, AuthorizerError, PrincipalRequirement, RelationshipResolver,
};

#[derive(Debug)]
pub struct DefaultAuthorizer<U, RR>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    RR: RelationshipResolver<Uow = U::Uow>,
{
    uow_factory: U,
    relationship_resolver: RR,
}

impl<U, RR> DefaultAuthorizer<U, RR>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    RR: RelationshipResolver<Uow = U::Uow>,
{
    pub fn new(uow_factory: U, relationship_resolver: RR) -> Self {
        Self {
            uow_factory,
            relationship_resolver,
        }
    }

    async fn check_principal_requirement(
        &self,
        principal: &Principal,
        principal_requirement: &PrincipalRequirement,
    ) -> Result<bool, AuthorizerError> {
        match principal_requirement {
            PrincipalRequirement::System => {
                if matches!(principal, Principal::System) {
                    Ok(true)
                } else if matches!(principal, Principal::Unavailable) {
                    Err(AuthorizerError::PrincipalUnavailable)
                } else {
                    Ok(false)
                }
            }
            PrincipalRequirement::Anonymous => {
                if matches!(principal, Principal::Anonymous) {
                    Ok(true)
                } else if matches!(principal, Principal::Unavailable) {
                    Err(AuthorizerError::PrincipalUnavailable)
                } else {
                    Ok(false)
                }
            }
            PrincipalRequirement::Authenticated => match principal {
                Principal::Authenticated { .. } => Ok(true),
                Principal::Anonymous => Ok(false),
                Principal::Unavailable => Err(AuthorizerError::PrincipalUnavailable),
                Principal::System => Ok(false),
            },
            PrincipalRequirement::AuthenticatedWithRelationship { requirement, .. } => {
                let subject = match principal {
                    Principal::Authenticated { subject } => subject,
                    Principal::Anonymous => return Ok(false),
                    Principal::Unavailable => return Err(AuthorizerError::PrincipalUnavailable),
                    Principal::System => return Ok(false),
                };

                let mut uow = self
                    .uow_factory
                    .begin()
                    .await
                    .map_err(AuthorizerError::backend)?;

                match self
                    .relationship_resolver
                    .satisfies(&mut uow, subject, requirement)
                    .await
                {
                    Ok(true) => {
                        uow.commit().await.map_err(AuthorizerError::backend)?;
                        Ok(true)
                    }
                    Ok(false) => {
                        uow.commit().await.map_err(AuthorizerError::backend)?;
                        Ok(false)
                    }
                    Err(operation_error) => {
                        let operation_error = uow
                            .rollback_with_operation_error(operation_error)
                            .await
                            .map_err(AuthorizerError::backend)?;
                        Err(AuthorizerError::backend(operation_error))
                    }
                }
            }
        }
    }
}

impl<U, RR> Authorizer for DefaultAuthorizer<U, RR>
where
    U: UnitOfWorkFactory,
    U::Uow: UnitOfWork,
    RR: RelationshipResolver<Uow = U::Uow>,
{
    async fn authorize(
        &self,
        principal: &Principal,
        authorization_plan: &AuthorizationPlan,
    ) -> Result<(), AuthorizerError> {
        if matches!(principal, Principal::Unavailable) {
            return Err(AuthorizerError::PrincipalUnavailable);
        }

        match authorization_plan {
            AuthorizationPlan::None => Ok(()),
            AuthorizationPlan::OnlyPrincipals(principal_requirements) => {
                for principal_requirement in principal_requirements {
                    if self
                        .check_principal_requirement(principal, principal_requirement)
                        .await?
                    {
                        return Ok(());
                    }
                }

                Err(AuthorizerError::Forbidden)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid::Uuid;

    use crate::authorization::DefaultRelationshipResolver;
    use crate::authorization::InMemoryAuthorizationModel;
    use crate::authorization::RelationshipChange;
    use crate::authorization::RelationshipStoreError;
    use crate::event::{AggregateIdValue, AggregateTypeOwned};
    use crate::request_context::Principal;
    use crate::unit_of_work::{
        UnitOfWork, UnitOfWorkError, UnitOfWorkFactory, UnitOfWorkFactoryError,
    };

    use crate::authorization::{
        AggregateRef, AuthorizationPlan, Authorizer, DefaultAuthorizer, PrincipalRequirement,
        RelationName, RelationRefOwned, RelationshipRequirement, RelationshipResolverConfig,
        RelationshipStore, RelationshipSubject, UsersetExprOwned,
    };
    use crate::projection::ProjectorDependencies;

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
    struct TestStore {
        map: HashMap<(AggregateRef, RelationRefOwned), Vec<RelationshipSubject>>,
    }

    impl RelationshipStore for TestStore {
        type Uow = TestUow;

        async fn apply_changes(
            &self,
            _uow: &mut TestUow,
            _changes: &[RelationshipChange],
        ) -> Result<(), RelationshipStoreError> {
            Ok(())
        }

        async fn read_aggregates_by_subject(
            &self,
            _uow: &mut TestUow,
            _subject: &RelationshipSubject,
            _relation: &RelationRefOwned,
        ) -> Result<Vec<AggregateRef>, RelationshipStoreError> {
            Ok(Vec::new())
        }

        async fn read_subjects_by_aggregate(
            &self,
            _uow: &mut TestUow,
            aggregate: &AggregateRef,
            relation: &RelationRefOwned,
        ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError> {
            Ok(self
                .map
                .get(&(aggregate.clone(), relation.clone()))
                .cloned()
                .unwrap_or_default())
        }
    }

    fn aggregate_type(value: &str) -> AggregateTypeOwned {
        value.parse().unwrap()
    }

    fn aggregate_ref(ty: &str, id: Uuid) -> AggregateRef {
        AggregateRef {
            aggregate_type: aggregate_type(ty),
            aggregate_id: AggregateIdValue::from(id),
        }
    }

    fn relation(value: &'static str) -> RelationName {
        RelationName::new(value)
    }

    fn relation_ref(aggregate_type_name: &str, relation_name: &'static str) -> RelationRefOwned {
        RelationRefOwned::new(
            aggregate_type(aggregate_type_name),
            relation(relation_name).into(),
        )
    }

    #[tokio::test]
    async fn allows_direct_subject_in_this_relation() {
        let doc = aggregate_ref("document", Uuid::from_u128(1));
        let user = aggregate_ref("user", Uuid::from_u128(2));

        let mut store = TestStore::default();
        store.map.insert(
            (doc.clone(), relation_ref("document", "editor")),
            vec![RelationshipSubject::Aggregate(user.clone())],
        );

        let mut models = InMemoryAuthorizationModel::new();
        models.define_expr(relation_ref("document", "editor"), UsersetExprOwned::This);

        let resolver =
            DefaultRelationshipResolver::new(store, models, RelationshipResolverConfig::default());
        let authorizer = DefaultAuthorizer::new(TestUowFactory, resolver);
        let principal = Principal::Authenticated { subject: user };

        authorizer
            .authorize(
                &principal,
                &AuthorizationPlan::OnlyPrincipals(vec![
                    PrincipalRequirement::AuthenticatedWithRelationship {
                        requirement: RelationshipRequirement::Check {
                            aggregate: doc,
                            relation: relation_ref("document", "editor"),
                        },
                        projector_dependencies: ProjectorDependencies::None,
                    },
                ]),
            )
            .await
            .expect("allowed");
    }
}
