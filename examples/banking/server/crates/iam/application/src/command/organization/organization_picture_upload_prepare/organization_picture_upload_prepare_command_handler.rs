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
use banking_iam_domain::{Organization, OrganizationPictureObjectName, OrganizationPictureRef};

use super::{
    OrganizationPictureUploadPrepareCommand, OrganizationPictureUploadPrepareCommandHandlerConfig,
    OrganizationPictureUploadPrepareCommandHandlerError, OrganizationPictureUploadPrepareOutput,
};
use crate::authorization::OrganizationProfileEditorRelation;
use crate::projection::{
    OrganizationOwnerRelationshipProjectorSpec, OrganizationRoleRelationshipProjectorSpec,
};

/// Handles `OrganizationPictureUploadPrepareCommand`.
pub struct OrganizationPictureUploadPrepareCommandHandler<OR, OUS>
where
    OR: Repository<Organization>,
    OUS: ObjectUploadSigner,
{
    organization_repository: OR,
    object_upload_signer: OUS,
    config: OrganizationPictureUploadPrepareCommandHandlerConfig,
}

impl<OR, OUS> OrganizationPictureUploadPrepareCommandHandler<OR, OUS>
where
    OR: Repository<Organization>,
    OUS: ObjectUploadSigner,
{
    pub fn new(
        organization_repository: OR,
        object_upload_signer: OUS,
        config: OrganizationPictureUploadPrepareCommandHandlerConfig,
    ) -> Self {
        Self {
            organization_repository,
            object_upload_signer,
            config,
        }
    }
}

impl<OR, OUS> CommandHandler for OrganizationPictureUploadPrepareCommandHandler<OR, OUS>
where
    OR: Repository<Organization>,
    OUS: ObjectUploadSigner,
{
    type Command = OrganizationPictureUploadPrepareCommand;
    type Output = OrganizationPictureUploadPrepareOutput;
    type ReplayOutput = OrganizationPictureUploadPrepareOutput;
    type Error = OrganizationPictureUploadPrepareCommandHandlerError;
    type Uow = OR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship {
                requirement: RelationshipRequirement::check::<Organization>(
                    command.organization_id,
                    OrganizationProfileEditorRelation::REF,
                ),
                projector_dependencies: ProjectorDependencies::Some(&[
                    OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                    OrganizationRoleRelationshipProjectorSpec::DESCRIPTOR,
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
        let Some(organization) = self
            .organization_repository
            .find(uow, command.organization_id)
            .await?
        else {
            return Err(OrganizationPictureUploadPrepareCommandHandlerError::OrganizationNotFound);
        };

        if organization.is_removed()? {
            return Err(OrganizationPictureUploadPrepareCommandHandlerError::OrganizationRemoved);
        }

        if command.content_length.value() > self.config.max_content_length().value() {
            return Err(OrganizationPictureUploadPrepareCommandHandlerError::ContentLengthTooLarge);
        }

        if !self
            .config
            .allowed_content_types()
            .contains(&command.content_type)
        {
            return Err(OrganizationPictureUploadPrepareCommandHandlerError::ContentTypeNotAllowed);
        }

        let picture_object_name = OrganizationPictureObjectName::new(command.organization_id);
        let picture = OrganizationPictureRef::object_name(picture_object_name.clone());
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
        let output = OrganizationPictureUploadPrepareOutput::new(picture, signed_upload_request);

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
        Organization, OrganizationDisplayName, OrganizationHandle, OrganizationId,
        OrganizationOwner, UserId,
    };
    use chrono::Duration;
    use uuid::Uuid;

    use super::{
        OrganizationPictureUploadPrepareCommand, OrganizationPictureUploadPrepareCommandHandler,
        OrganizationPictureUploadPrepareCommandHandlerConfig,
        OrganizationPictureUploadPrepareCommandHandlerError,
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
    struct TestOrganizationRepository {
        organization: Arc<Mutex<Option<Organization>>>,
    }

    impl TestOrganizationRepository {
        fn new(organization: Organization) -> Self {
            Self {
                organization: Arc::new(Mutex::new(Some(organization))),
            }
        }
    }

    impl Repository<Organization> for TestOrganizationRepository {
        type Uow = TestUow;

        async fn find(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
        ) -> Result<Option<Organization>, RepositoryError<Organization>> {
            Ok(self.organization.lock().expect("lock").clone())
        }

        async fn find_at_version(
            &self,
            _uow: &mut Self::Uow,
            _id: OrganizationId,
            _at: Option<appletheia::domain::AggregateVersion>,
        ) -> Result<Option<Organization>, RepositoryError<Organization>> {
            Ok(self.organization.lock().expect("lock").clone())
        }

        async fn find_by_unique_value(
            &self,
            _uow: &mut Self::Uow,
            _unique_key: appletheia::domain::UniqueKey,
            _unique_value: &appletheia::domain::UniqueValue,
        ) -> Result<Option<Organization>, RepositoryError<Organization>> {
            Ok(None)
        }

        async fn save(
            &self,
            _uow: &mut Self::Uow,
            _request_context: &RequestContext,
            aggregate: &mut Organization,
        ) -> Result<(), RepositoryError<Organization>> {
            *self.organization.lock().expect("lock") = Some(aggregate.clone());
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

    fn request_context() -> RequestContext {
        RequestContext::new(
            CorrelationId::from(Uuid::now_v7()),
            MessageId::new(),
            Principal::Authenticated {
                subject: AggregateRef::new(
                    appletheia::application::event::AggregateTypeOwned::try_from("user")
                        .expect("aggregate type should be valid"),
                    appletheia::application::event::AggregateIdValue::from(Uuid::now_v7()),
                ),
            },
        )
        .expect("request context should be valid")
    }

    fn organization() -> Organization {
        let mut organization = Organization::default();
        organization
            .create(
                OrganizationOwner::User(UserId::new()),
                OrganizationHandle::try_from("acme-labs").expect("handle should be valid"),
                OrganizationDisplayName::try_from("Acme Labs")
                    .expect("display name should be valid"),
                None,
                None,
                None,
            )
            .expect("organization should create");
        organization
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
    fn authorization_plan_requires_organization_profile_editor_relationship() {
        let repository = TestOrganizationRepository::new(organization());
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let handler = OrganizationPictureUploadPrepareCommandHandler::new(
            repository,
            TestObjectUploadSigner::new(signed_upload_request(expires_in)),
            OrganizationPictureUploadPrepareCommandHandlerConfig::new(
                ObjectBucketName::new("pictures".to_owned()).expect("bucket name should be valid"),
                expires_in,
                ObjectContentLength::new(5 * 1024 * 1024),
                allowed_content_types(),
            ),
        );
        let organization_id = OrganizationId::new();

        let plan = handler
            .authorization_plan(&OrganizationPictureUploadPrepareCommand {
                organization_id,
                content_type: ObjectContentType::png(),
                content_length: ObjectContentLength::new(1024),
                checksum: checksum(),
            })
            .expect("authorization plan should build");

        assert_eq!(
            plan,
            AuthorizationPlan::OnlyPrincipals(vec![
                PrincipalRequirement::AuthenticatedWithRelationship {
                    requirement: RelationshipRequirement::check::<Organization>(
                        organization_id,
                        crate::authorization::OrganizationProfileEditorRelation::REF,
                    ),
                    projector_dependencies: ProjectorDependencies::Some(&[
                        crate::projection::OrganizationOwnerRelationshipProjectorSpec::DESCRIPTOR,
                        crate::projection::OrganizationRoleRelationshipProjectorSpec::DESCRIPTOR,
                    ]),
                },
            ])
        );
    }

    #[tokio::test]
    async fn handle_returns_picture_ref_and_signed_upload_request() {
        let organization = organization();
        let organization_id = organization
            .aggregate_id()
            .expect("organization id should exist");
        let repository = TestOrganizationRepository::new(organization);
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let signer = TestObjectUploadSigner::new(signed_upload_request(expires_in));
        let signer_requests = signer.request.clone();
        let handler = OrganizationPictureUploadPrepareCommandHandler::new(
            repository,
            signer,
            OrganizationPictureUploadPrepareCommandHandlerConfig::new(
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
                &request_context(),
                &OrganizationPictureUploadPrepareCommand {
                    organization_id,
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
        let expected_prefix = format!("organizations/{organization_id}/pictures/");
        assert!(request.object_name().as_str().starts_with(&expected_prefix));
        let picture_id = request
            .object_name()
            .as_str()
            .strip_prefix(&expected_prefix)
            .expect("object name should have organization picture prefix");
        Uuid::parse_str(picture_id).expect("picture ID should be a UUID");
    }

    #[tokio::test]
    async fn handle_rejects_removed_organization() {
        let mut organization = organization();
        organization.remove().expect("organization should remove");
        let organization_id = organization
            .aggregate_id()
            .expect("organization id should exist");
        let repository = TestOrganizationRepository::new(organization);
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let handler = OrganizationPictureUploadPrepareCommandHandler::new(
            repository,
            TestObjectUploadSigner::new(signed_upload_request(expires_in)),
            OrganizationPictureUploadPrepareCommandHandlerConfig::new(
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
                &request_context(),
                &OrganizationPictureUploadPrepareCommand {
                    organization_id,
                    content_type: ObjectContentType::png(),
                    content_length: ObjectContentLength::new(1024),
                    checksum: checksum(),
                },
            )
            .await
            .expect_err("removed organization should be rejected");

        assert!(matches!(
            error,
            OrganizationPictureUploadPrepareCommandHandlerError::OrganizationRemoved
        ));
    }

    #[tokio::test]
    async fn handle_rejects_content_length_over_maximum() {
        let organization = organization();
        let organization_id = organization
            .aggregate_id()
            .expect("organization id should exist");
        let repository = TestOrganizationRepository::new(organization);
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let handler = OrganizationPictureUploadPrepareCommandHandler::new(
            repository,
            TestObjectUploadSigner::new(signed_upload_request(expires_in)),
            OrganizationPictureUploadPrepareCommandHandlerConfig::new(
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
                &request_context(),
                &OrganizationPictureUploadPrepareCommand {
                    organization_id,
                    content_type: ObjectContentType::png(),
                    content_length: ObjectContentLength::new(1024),
                    checksum: checksum(),
                },
            )
            .await
            .expect_err("oversized content should be rejected");

        assert!(matches!(
            error,
            OrganizationPictureUploadPrepareCommandHandlerError::ContentLengthTooLarge
        ));
    }

    #[tokio::test]
    async fn handle_rejects_disallowed_content_type() {
        let organization = organization();
        let organization_id = organization
            .aggregate_id()
            .expect("organization id should exist");
        let repository = TestOrganizationRepository::new(organization);
        let expires_in =
            ObjectUploadExpiresIn::new(Duration::minutes(10)).expect("expiration should be valid");
        let handler = OrganizationPictureUploadPrepareCommandHandler::new(
            repository,
            TestObjectUploadSigner::new(signed_upload_request(expires_in)),
            OrganizationPictureUploadPrepareCommandHandlerConfig::new(
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
                &request_context(),
                &OrganizationPictureUploadPrepareCommand {
                    organization_id,
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
            OrganizationPictureUploadPrepareCommandHandlerError::ContentTypeNotAllowed
        ));
    }
}
