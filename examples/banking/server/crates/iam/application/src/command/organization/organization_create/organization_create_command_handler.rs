use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use appletheia::domain::{Aggregate, UniqueValue};
use banking_iam_domain::OrganizationError;
use banking_iam_domain::{
    Organization, OrganizationHandle, OrganizationOwner, OrganizationState, User,
};

use super::{
    OrganizationCreateCommand, OrganizationCreateCommandHandlerError, OrganizationCreateOutput,
};
use crate::authorization::UserOwnerRelation;

pub struct OrganizationCreateCommandHandler<R>
where
    R: Repository,
{
    repository: R,
}

impl<R> OrganizationCreateCommandHandler<R>
where
    R: Repository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn handle_unique_value(
        handle: &OrganizationHandle,
    ) -> Result<UniqueValue, OrganizationCreateCommandHandlerError> {
        Ok(UniqueValue::from_strings([handle.as_ref()])?)
    }
}

impl<R> CommandHandler for OrganizationCreateCommandHandler<R>
where
    R: Repository,
{
    type Command = OrganizationCreateCommand;
    type Output = OrganizationCreateOutput;
    type Error = OrganizationCreateCommandHandlerError;
    type Uow = R::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        let OrganizationOwner::User(owner) = command.owner;

        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                User,
            >(
                owner, UserOwnerRelation::REF
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let OrganizationCreateCommand {
            owner,
            handle,
            display_name,
            description,
            website_url,
            picture,
        } = command.clone();

        let mut organization = Organization::new();
        let organization_id = organization.aggregate_id();

        let unique_value = Self::handle_unique_value(&handle)?;
        let handle_is_taken = self
            .repository
            .find_by_unique_value::<Organization>(uow, OrganizationState::HANDLE_KEY, &unique_value)
            .await?
            .is_some();
        if handle_is_taken {
            return Err(OrganizationError::HandleAlreadyTaken.into());
        }

        organization.create(owner, handle, display_name)?;
        if let Some(description) = description {
            organization.set_description(Some(description))?;
        }
        if let Some(website_url) = website_url {
            organization.set_website_url(Some(website_url))?;
        }
        if let Some(picture) = picture {
            organization.set_picture(Some(picture))?;
        }

        self.repository
            .save(uow, request_context, &mut organization)
            .await?;

        Ok(OrganizationCreateOutput { organization_id })
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::aggregate::SerializedAggregate;
    use appletheia::domain::{AggregateId, Event};
    use std::sync::{Arc, Mutex};

    use appletheia::application::aggregate::AggregateRef;
    use appletheia::application::authorization::{
        AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;

    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_domain::{
        Organization, OrganizationDisplayName, OrganizationHandle, OrganizationOwner, User, UserId,
    };
    use uuid::Uuid;

    use super::{
        OrganizationCreateCommand, OrganizationCreateCommandHandler, OrganizationCreateOutput,
    };

    fn display_name() -> OrganizationDisplayName {
        OrganizationDisplayName::try_from("Acme Labs").expect("display name should be valid")
    }

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
    struct TestRepository {
        organization: Arc<Mutex<Option<Organization>>>,
    }

    impl TestRepository {
        fn copy_aggregate<S: Aggregate, T: Aggregate>(source: &S) -> T {
            let mut restored = SerializedAggregate::try_from_aggregate(source)
                .expect("test aggregate should serialize")
                .try_to_aggregate::<T>()
                .expect("test aggregate types should match");
            for event in source.uncommitted_events() {
                let id = T::Id::try_from_uuid(event.aggregate_id().value()).unwrap();
                let payload =
                    serde_json::from_value(serde_json::to_value(event.payload()).unwrap()).unwrap();
                restored
                    .core_mut()
                    .record_uncommitted_event(Event::from_persisted(
                        event.id(),
                        id,
                        event.aggregate_version(),
                        payload,
                        event.occurred_at(),
                    ));
            }
            restored
        }

        fn new(organization: Organization) -> Self {
            Self {
                organization: Arc::new(Mutex::new(Some(organization))),
            }
        }
    }

    impl Repository for TestRepository {
        type Uow = TestUow;

        async fn read<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            id: A::Id,
        ) -> Result<A, RepositoryError<A>> {
            self.organization
                .lock()
                .expect("lock")
                .as_ref()
                .map(|stored| Self::copy_aggregate::<_, A>(stored))
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: A::TYPE,
                    aggregate_id: id,
                })
        }

        async fn read_at_version<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            id: A::Id,
            _at: appletheia::domain::AggregateVersion,
        ) -> Result<A, RepositoryError<A>> {
            self.organization
                .lock()
                .expect("lock")
                .as_ref()
                .map(|stored| Self::copy_aggregate::<_, A>(stored))
                .ok_or_else(|| RepositoryError::NotFound {
                    aggregate_type: A::TYPE,
                    aggregate_id: id,
                })
        }

        async fn find_by_unique_value<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<A>, RepositoryError<A>> {
            Ok(self
                .organization
                .lock()
                .expect("lock")
                .as_ref()
                .map(|stored| Self::copy_aggregate::<_, A>(stored)))
        }

        async fn save<A: Aggregate>(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut A,
        ) -> Result<(), RepositoryError<A>> {
            *self.organization.lock().expect("lock") =
                Some(Self::copy_aggregate::<_, Organization>(aggregate));
            Ok(())
        }
    }

    fn request_context() -> (RequestContext, UserId) {
        let user_id = UserId::new();
        let subject = AggregateRef::from_id::<User>(user_id);

        (
            RequestContext::new(
                CorrelationId::from(Uuid::now_v7()),
                MessageId::new(),
                Principal::Authenticated { subject },
            )
            .expect("request context should be valid"),
            user_id,
        )
    }

    #[test]
    fn authorization_plan_requires_user_owner_relationship() {
        let repository = TestRepository::default();
        let handler = OrganizationCreateCommandHandler::new(repository);

        let owner = UserId::new();
        let plan = handler
            .authorization_plan(&OrganizationCreateCommand {
                owner: OrganizationOwner::User(owner),
                handle: OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                display_name: display_name(),
                description: None,
                website_url: None,
                picture: None,
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship(
                    RelationshipRequirement::check::<User>(
                        owner,
                        crate::authorization::UserOwnerRelation::REF,
                    )
                ),
            ])
        );
    }

    #[tokio::test]
    async fn handle_creates_organization_and_returns_id() {
        use banking_iam_domain::{
            OrganizationDescription, OrganizationEventPayload, OrganizationPictureRef,
            OrganizationPictureUrl, OrganizationWebsiteUrl,
        };

        for with_optional_fields in [false, true] {
            let repository = TestRepository::default();
            let handler = OrganizationCreateCommandHandler::new(repository.clone());
            let mut uow = TestUow;
            let (request_context, user_id) = request_context();

            let output = handler
                .handle(
                    &mut uow,
                    &request_context,
                    &OrganizationCreateCommand {
                        owner: OrganizationOwner::User(user_id),
                        handle: OrganizationHandle::try_from("acme-labs")
                            .expect("handle should be valid"),
                        display_name: display_name(),
                        description: with_optional_fields
                            .then(|| OrganizationDescription::try_from("Description").unwrap()),
                        website_url: with_optional_fields.then(|| {
                            OrganizationWebsiteUrl::try_from("https://example.com").unwrap()
                        }),
                        picture: with_optional_fields.then(|| {
                            OrganizationPictureRef::external_url(
                                OrganizationPictureUrl::try_from("https://example.com/picture.png")
                                    .unwrap(),
                            )
                        }),
                    },
                )
                .await
                .expect("command should succeed");

            let saved = repository
                .organization
                .lock()
                .expect("lock")
                .clone()
                .expect("organization should be saved");

            assert_eq!(
                output,
                OrganizationCreateOutput {
                    organization_id: saved.aggregate_id(),
                }
            );
            assert_eq!(
                serde_json::to_value(&output).expect("output should serialize"),
                serde_json::json!({ "organization_id": saved.aggregate_id() })
            );
            assert_eq!(
                saved.display_name().expect("display name should exist"),
                &display_name()
            );
            assert_eq!(
                saved.handle().expect("handle should exist"),
                &OrganizationHandle::try_from("acme-labs").expect("handle should be valid")
            );
            assert_eq!(
                saved.owner().expect("owner should exist"),
                OrganizationOwner::User(user_id)
            );
            assert_eq!(
                saved.uncommitted_events().len(),
                if with_optional_fields { 4 } else { 1 }
            );
            assert_eq!(saved.description().unwrap().is_some(), with_optional_fields);
            assert_eq!(saved.website_url().unwrap().is_some(), with_optional_fields);
            assert_eq!(saved.picture().unwrap().is_some(), with_optional_fields);
            if with_optional_fields {
                assert!(matches!(
                    saved.uncommitted_events()[1].payload(),
                    OrganizationEventPayload::DescriptionSet { .. }
                ));
                assert!(matches!(
                    saved.uncommitted_events()[2].payload(),
                    OrganizationEventPayload::WebsiteUrlSet { .. }
                ));
                assert!(matches!(
                    saved.uncommitted_events()[3].payload(),
                    OrganizationEventPayload::PictureSet {
                        old_picture: None,
                        ..
                    }
                ));
            }
        }
    }

    #[tokio::test]
    async fn handle_returns_error_when_handle_is_taken() {
        let mut existing = Organization::new();
        existing
            .create(
                OrganizationOwner::User(UserId::new()),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                display_name(),
            )
            .expect("existing organization should be created");
        let repository = TestRepository::new(existing);
        let handler = OrganizationCreateCommandHandler::new(repository.clone());
        let mut uow = TestUow;
        let (request_context, user_id) = request_context();

        handler
            .handle(
                &mut uow,
                &request_context,
                &OrganizationCreateCommand {
                    owner: OrganizationOwner::User(user_id),
                    handle: OrganizationHandle::try_from("acme-labs")
                        .expect("handle should be valid"),
                    display_name: display_name(),
                    description: None,
                    website_url: None,
                    picture: None,
                },
            )
            .await
            .expect_err("duplicate handle should return an error");
    }
}
