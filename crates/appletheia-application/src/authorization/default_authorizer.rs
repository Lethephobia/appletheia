use crate::request_context::Principal;
use crate::unit_of_work::{UnitOfWork, UnitOfWorkFactory};

use super::{Authorizer, AuthorizerError, RelationshipRequirement, RelationshipResolver};

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
        requirement: &RelationshipRequirement,
    ) -> Result<(), AuthorizerError> {
        if matches!(requirement, RelationshipRequirement::None) {
            return Ok(());
        }

        let subject = match principal {
            Principal::Authenticated { subject } => subject,
            Principal::Anonymous => return Err(AuthorizerError::Unauthenticated),
            Principal::Unavailable => return Err(AuthorizerError::PrincipalUnavailable),
            Principal::System => return Ok(()),
        };

        let mut uow = self
            .uow_factory
            .begin()
            .await
            .map_err(AuthorizerError::backend)?;

        let operation = async {
            self.relationship_resolver
                .satisfies(&mut uow, subject, requirement)
                .await
        };

        match operation.await {
            Ok(true) => {
                uow.commit().await.map_err(AuthorizerError::backend)?;
                Ok(())
            }
            Ok(false) => {
                uow.commit().await.map_err(AuthorizerError::backend)?;
                Err(AuthorizerError::Forbidden)
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid::Uuid;

    use crate::authorization::DefaultRelationshipResolver;
    use crate::authorization::RelationshipChange;
    use crate::authorization::RelationshipStoreError;
    use crate::event::{AggregateIdValue, AggregateTypeOwned};
    use crate::request_context::Principal;
    use crate::unit_of_work::{
        UnitOfWork, UnitOfWorkError, UnitOfWorkFactory, UnitOfWorkFactoryError,
    };

    use crate::authorization::{
        AggregateRef, AuthorizationModel, AuthorizationTypeDefinition, Authorizer,
        DefaultAuthorizer, RelationName, RelationshipRequirement, RelationshipResolverConfig,
        RelationshipStore, RelationshipSubject, UsersetExpr,
    };

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
        map: HashMap<(AggregateRef, RelationName), Vec<RelationshipSubject>>,
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
            _relation: &RelationName,
            _aggregate_type: Option<&AggregateTypeOwned>,
        ) -> Result<Vec<AggregateRef>, RelationshipStoreError> {
            Ok(Vec::new())
        }

        async fn read_subjects_by_aggregate(
            &self,
            _uow: &mut TestUow,
            aggregate: &AggregateRef,
            relation: &RelationName,
        ) -> Result<Vec<RelationshipSubject>, RelationshipStoreError> {
            Ok(self
                .map
                .get(&(aggregate.clone(), relation.clone()))
                .cloned()
                .unwrap_or_default())
        }
    }

    #[derive(Clone, Default)]
    struct TestModels {
        models: HashMap<AggregateTypeOwned, AuthorizationTypeDefinition>,
    }

    impl AuthorizationModel for TestModels {
        fn type_definition_for(
            &self,
            aggregate_type: &AggregateTypeOwned,
        ) -> Option<&AuthorizationTypeDefinition> {
            self.models.get(aggregate_type)
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

    fn relation(value: &str) -> RelationName {
        value.parse().unwrap()
    }

    #[tokio::test]
    async fn allows_direct_subject_in_this_relation() {
        let doc = aggregate_ref("document", Uuid::from_u128(1));
        let user = aggregate_ref("user", Uuid::from_u128(2));

        let mut store = TestStore::default();
        store.map.insert(
            (doc.clone(), relation("editor")),
            vec![RelationshipSubject::Aggregate(user.clone())],
        );

        let mut model = AuthorizationTypeDefinition::default();
        model.define_relation(relation("editor"), UsersetExpr::This);

        let mut models = TestModels::default();
        models.models.insert(doc.aggregate_type.clone(), model);

        let resolver =
            DefaultRelationshipResolver::new(store, models, RelationshipResolverConfig::default());
        let authorizer = DefaultAuthorizer::new(TestUowFactory, resolver);
        let principal = Principal::Authenticated { subject: user };

        authorizer
            .authorize(
                &principal,
                &RelationshipRequirement::Check {
                    aggregate: doc,
                    relation: relation("editor"),
                },
            )
            .await
            .expect("allowed");
    }
}
