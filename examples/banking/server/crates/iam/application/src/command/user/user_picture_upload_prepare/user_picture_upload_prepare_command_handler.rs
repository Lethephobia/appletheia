use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::object_storage::{
    ObjectName, ObjectUploadRequest, ObjectUploadSigner,
};
use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_iam_domain::{User, UserPictureObjectName, UserPictureRef};

use super::{
    UserPictureUploadPrepareCommand, UserPictureUploadPrepareCommandHandlerConfig,
    UserPictureUploadPrepareCommandHandlerError, UserPictureUploadPrepareOutput,
};
use crate::authorization::UserPictureChangerRelation;
use crate::projection::UserOwnerRelationshipProjectorSpec;

/// Handles `UserPictureUploadPrepareCommand`.
pub struct UserPictureUploadPrepareCommandHandler<UR, OUS>
where
    UR: Repository<User>,
    OUS: ObjectUploadSigner,
{
    user_repository: UR,
    object_upload_signer: OUS,
    config: UserPictureUploadPrepareCommandHandlerConfig,
}

impl<UR, OUS> UserPictureUploadPrepareCommandHandler<UR, OUS>
where
    UR: Repository<User>,
    OUS: ObjectUploadSigner,
{
    pub fn new(
        user_repository: UR,
        object_upload_signer: OUS,
        config: UserPictureUploadPrepareCommandHandlerConfig,
    ) -> Self {
        Self {
            user_repository,
            object_upload_signer,
            config,
        }
    }
}

impl<UR, OUS> CommandHandler for UserPictureUploadPrepareCommandHandler<UR, OUS>
where
    UR: Repository<User>,
    OUS: ObjectUploadSigner,
{
    type Command = UserPictureUploadPrepareCommand;
    type Output = UserPictureUploadPrepareOutput;
    type ReplayOutput = UserPictureUploadPrepareOutput;
    type Error = UserPictureUploadPrepareCommandHandlerError;
    type Uow = UR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<User>(
                    command.user_id,
                    UserPictureChangerRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    UserOwnerRelationshipProjectorSpec::DESCRIPTOR,
                ]),
            },
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(user) = self.user_repository.find(uow, command.user_id).await? else {
            return Err(UserPictureUploadPrepareCommandHandlerError::UserNotFound);
        };

        if user.is_removed()? {
            return Err(UserPictureUploadPrepareCommandHandlerError::UserRemoved);
        }

        if user.is_inactive()? {
            return Err(UserPictureUploadPrepareCommandHandlerError::UserInactive);
        }

        if command.content_length.value() > self.config.max_content_length().value() {
            return Err(UserPictureUploadPrepareCommandHandlerError::ContentLengthTooLarge);
        }

        if !self
            .config
            .allowed_content_types()
            .contains(&command.content_type)
        {
            return Err(UserPictureUploadPrepareCommandHandlerError::ContentTypeNotAllowed);
        }

        let picture_object_name = UserPictureObjectName::new(command.user_id);
        let picture = UserPictureRef::object_name(picture_object_name.clone());
        let object_name = ObjectName::new(picture_object_name.value().to_owned())?;
        let request = ObjectUploadRequest::new(
            self.config.bucket_name().clone(),
            object_name,
            command.content_type.clone(),
            self.config.expires_in(),
        )
        .with_content_length(command.content_length)
        .with_checksum(command.checksum.clone());
        let signed_upload_request = self.object_upload_signer.sign(request).await?;
        let output = UserPictureUploadPrepareOutput::new(picture, signed_upload_request);

        Ok(CommandHandled::same(output))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use appletheia::application::authorization::{
        AggregateRef, AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
    };
    use appletheia::application::command::CommandHandler;
    use appletheia::application::object_storage::{
        ObjectBucketName, ObjectChecksum, ObjectChecksumAlgorithm, ObjectChecksumValue,
        ObjectContentLength, ObjectContentType, ObjectContentTypes, ObjectUploadExpiresIn,
        ObjectUploadHeaders, ObjectUploadRequest, ObjectUploadSigner, ObjectUploadSignerError,
        SignedObjectUploadRequest, SignedObjectUploadUrl,
    };
    use appletheia::application::projection::{ProjectorDependencies, ProjectorSpec};
    use appletheia::application::repository::{Repository, RepositoryError};
    use appletheia::application::request_context::{
        CorrelationId, MessageId, Principal, RequestContext,
    };
    use appletheia::application::unit_of_work::{UnitOfWork, UnitOfWorkError};
    use appletheia::domain::Aggregate;
    use banking_iam_domain::{
        User, UserId, UserIdentity, UserIdentityProvider, UserIdentitySubject,
    };
    use chrono::Duration;
    use uuid::Uuid;

    use super::{
        UserPictureUploadPrepareCommand, UserPictureUploadPrepareCommandHandler,
        UserPictureUploadPrepareCommandHandlerConfig, UserPictureUploadPrepareCommandHandlerError,
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
    struct TestUserRepository {
        user: Arc<Mutex<Option<User>>>,
    }

    impl TestUserRepository {
        fn new(user: User) -> Self {
            Self {
                user: Arc::new(Mutex::new(Some(user))),
            }
        }
    }

    impl Repository<User> for TestUserRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: UserId,
        ) -> Result<Option<User>, RepositoryError<User>> {
            Ok(self.user.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: UserId,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<User>, RepositoryError<User>> {
            Ok(self.user.lock().expect("lock").clone())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<User>, RepositoryError<User>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut User,
        ) -> Result<(), RepositoryError<User>> {
            *self.user.lock().expect("lock") = Some(aggregate.clone());
            Ok(())
        }
    }

    #[derive(Clone)]
    struct TestObjectUploadSigner {
        request: Arc<Mutex<Option<ObjectUploadRequest>>>,
        signed_upload_request: SignedObjectUploadRequest,
    }

    impl TestObjectUploadSigner {
        fn new(signed_upload_request: SignedObjectUploadRequest) -> Self {
            Self {
                request: Arc::new(Mutex::new(None)),
                signed_upload_request,
            }
        }
    }

    impl ObjectUploadSigner for TestObjectUploadSigner {
        async fn sign(
            &self,
            request: ObjectUploadRequest,
        ) -> Result<SignedObjectUploadRequest, ObjectUploadSignerError> {
            *self.request.lock().expect("lock") = Some(request);
            Ok(self.signed_upload_request.clone())
        }
    }

    fn request_context(user_id: UserId) -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Authenticated {
                subject: AggregateRef::from_id::<User>(user_id),
            },
        )
        .expect("request context should be valid")
    }

    fn registered_user() -> User {
        let mut user = User::default();
        user.register(UserIdentity::new(
            UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
            None,
        ))
        .expect("user should register");
        user
    }

    fn signed_upload_request(expires_in: ObjectUploadExpiresIn) -> SignedObjectUploadRequest {
        SignedObjectUploadRequest::new(
            appletheia::application::object_storage::ObjectUploadMethod::Put,
            SignedObjectUploadUrl::try_from("https://storage.example.com/upload")
                .expect("signed URL should be valid"),
            expires_in,
            ObjectUploadHeaders::default(),
        )
    }

    fn checksum() -> ObjectChecksum {
        ObjectChecksum::new(
            ObjectChecksumAlgorithm::Md5,
            ObjectChecksumValue::new("kAFQmDzST7DWlj99KOF/cg==".to_owned())
                .expect("checksum should be valid"),
        )
    }

    fn allowed_content_types() -> ObjectContentTypes {
        ObjectContentTypes::from([
            ObjectContentType::png(),
            ObjectContentType::jpeg(),
            ObjectContentType::webp(),
        ])
    }

    #[test]
    fn authorization_plan_requires_user_picture_changer_relationship() {
        let repository = TestUserRepository::new(registered_user());
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let handler = UserPictureUploadPrepareCommandHandler::new(
            repository,
            TestObjectUploadSigner::new(signed_upload_request(expires_in)),
            UserPictureUploadPrepareCommandHandlerConfig::new(
                ObjectBucketName::new("pictures".to_owned()).expect("bucket name should be valid"),
                expires_in,
                ObjectContentLength::new(5 * 1024 * 1024),
                allowed_content_types(),
            ),
        );
        let user_id = UserId::new();

        let plan = handler
            .authorization_plan(&UserPictureUploadPrepareCommand {
                user_id,
                content_type: ObjectContentType::png(),
                content_length: ObjectContentLength::new(1024),
                checksum: checksum(),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::check::<User>(
                        user_id,
                        crate::authorization::UserPictureChangerRelation::REF,
                    ),
                    projector_dependencies: ProjectorDependencies::Some(&[
                        crate::projection::UserOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_returns_picture_ref_and_signed_upload_request() {
        let user = registered_user();
        let user_id = user.aggregate_id().expect("user id should exist");
        let repository = TestUserRepository::new(user);
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let signer = TestObjectUploadSigner::new(signed_upload_request(expires_in));
        let signer_requests = signer.request.clone();
        let handler = UserPictureUploadPrepareCommandHandler::new(
            repository,
            signer,
            UserPictureUploadPrepareCommandHandlerConfig::new(
                ObjectBucketName::new("pictures".to_owned()).expect("bucket name should be valid"),
                expires_in,
                ObjectContentLength::new(5 * 1024 * 1024),
                allowed_content_types(),
            ),
        );
        let mut uow = TestUow;

        let handled = handler
            .handle(
                &mut uow,
                &request_context(user_id),
                &UserPictureUploadPrepareCommand {
                    user_id,
                    content_type: ObjectContentType::png(),
                    content_length: ObjectContentLength::new(1024),
                    checksum: checksum(),
                },
            )
            .await
            .expect("command should succeed");

        let output = handled.into_output();
        let request = signer_requests
            .lock()
            .expect("lock")
            .clone()
            .expect("signer should receive request");

        assert_eq!(
            output.signed_upload_request,
            signed_upload_request(expires_in)
        );
        assert_eq!(request.bucket_name().as_str(), "pictures");
        assert_eq!(request.content_type().as_str(), "image/png");
        assert_eq!(
            request.content_length().map(|value| value.value()),
            Some(1024)
        );
        assert_eq!(request.expires_in(), expires_in);
        assert_eq!(request.checksum(), Some(&checksum()));
        assert_eq!(
            output.picture.as_object_name().map(|value| value.value()),
            Some(request.object_name().as_str())
        );
        let expected_prefix = format!("users/{user_id}/pictures/");
        assert!(request.object_name().as_str().starts_with(&expected_prefix));
        let picture_id = request
            .object_name()
            .as_str()
            .strip_prefix(&expected_prefix)
            .expect("object name should have user picture prefix");
        Uuid::parse_str(picture_id).expect("picture ID should be a UUID");
    }

    #[tokio::test]
    async fn handle_rejects_inactive_user() {
        let mut user = registered_user();
        user.deactivate().expect("user should deactivate");
        let user_id = user.aggregate_id().expect("user id should exist");
        let repository = TestUserRepository::new(user);
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let handler = UserPictureUploadPrepareCommandHandler::new(
            repository,
            TestObjectUploadSigner::new(signed_upload_request(expires_in)),
            UserPictureUploadPrepareCommandHandlerConfig::new(
                ObjectBucketName::new("pictures".to_owned()).expect("bucket name should be valid"),
                expires_in,
                ObjectContentLength::new(5 * 1024 * 1024),
                allowed_content_types(),
            ),
        );
        let mut uow = TestUow;

        let error = handler
            .handle(
                &mut uow,
                &request_context(user_id),
                &UserPictureUploadPrepareCommand {
                    user_id,
                    content_type: ObjectContentType::png(),
                    content_length: ObjectContentLength::new(1024),
                    checksum: checksum(),
                },
            )
            .await
            .expect_err("inactive user should be rejected");

        assert!(matches!(
            error,
            UserPictureUploadPrepareCommandHandlerError::UserInactive
        ));
    }

    #[tokio::test]
    async fn handle_rejects_content_length_over_maximum() {
        let user = registered_user();
        let user_id = user.aggregate_id().expect("user id should exist");
        let repository = TestUserRepository::new(user);
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let handler = UserPictureUploadPrepareCommandHandler::new(
            repository,
            TestObjectUploadSigner::new(signed_upload_request(expires_in)),
            UserPictureUploadPrepareCommandHandlerConfig::new(
                ObjectBucketName::new("pictures".to_owned()).expect("bucket name should be valid"),
                expires_in,
                ObjectContentLength::new(512),
                allowed_content_types(),
            ),
        );
        let mut uow = TestUow;

        let error = handler
            .handle(
                &mut uow,
                &request_context(user_id),
                &UserPictureUploadPrepareCommand {
                    user_id,
                    content_type: ObjectContentType::png(),
                    content_length: ObjectContentLength::new(1024),
                    checksum: checksum(),
                },
            )
            .await
            .expect_err("oversized content should be rejected");

        assert!(matches!(
            error,
            UserPictureUploadPrepareCommandHandlerError::ContentLengthTooLarge
        ));
    }

    #[tokio::test]
    async fn handle_rejects_disallowed_content_type() {
        let user = registered_user();
        let user_id = user.aggregate_id().expect("user id should exist");
        let repository = TestUserRepository::new(user);
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let handler = UserPictureUploadPrepareCommandHandler::new(
            repository,
            TestObjectUploadSigner::new(signed_upload_request(expires_in)),
            UserPictureUploadPrepareCommandHandlerConfig::new(
                ObjectBucketName::new("pictures".to_owned()).expect("bucket name should be valid"),
                expires_in,
                ObjectContentLength::new(5 * 1024 * 1024),
                allowed_content_types(),
            ),
        );
        let mut uow = TestUow;

        let error = handler
            .handle(
                &mut uow,
                &request_context(user_id),
                &UserPictureUploadPrepareCommand {
                    user_id,
                    content_type: ObjectContentType::try_from("image/gif")
                        .expect("content type should be valid"),
                    content_length: ObjectContentLength::new(1024),
                    checksum: checksum(),
                },
            )
            .await
            .expect_err("disallowed content type should be rejected");

        assert!(matches!(
            error,
            UserPictureUploadPrepareCommandHandlerError::ContentTypeNotAllowed
        ));
    }
}
