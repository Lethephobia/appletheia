use std::marker::PhantomData;

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::object_storage::{
    ObjectBucketName, ObjectDeleteRequest, ObjectDeleter, ObjectName,
};
use appletheia::application::request_context::RequestContext;
use appletheia::application::unit_of_work::UnitOfWork;

use super::{
    OrganizationPictureObjectDeleteCommand, OrganizationPictureObjectDeleteCommandHandlerError,
    OrganizationPictureObjectDeleteOutput,
};

/// Handles `OrganizationPictureObjectDeleteCommand`.
pub struct OrganizationPictureObjectDeleteCommandHandler<OD, U>
where
    OD: ObjectDeleter,
    U: UnitOfWork + Sync,
{
    object_deleter: OD,
    bucket_name: ObjectBucketName,
    _uow: PhantomData<U>,
}

impl<OD, U> OrganizationPictureObjectDeleteCommandHandler<OD, U>
where
    OD: ObjectDeleter,
    U: UnitOfWork + Sync,
{
    pub fn new(object_deleter: OD, bucket_name: ObjectBucketName) -> Self {
        Self {
            object_deleter,
            bucket_name,
            _uow: PhantomData,
        }
    }
}

impl<OD, U> CommandHandler for OrganizationPictureObjectDeleteCommandHandler<OD, U>
where
    OD: ObjectDeleter,
    U: UnitOfWork + Sync,
{
    type Command = OrganizationPictureObjectDeleteCommand;
    type Output = OrganizationPictureObjectDeleteOutput;
    type ReplayOutput = OrganizationPictureObjectDeleteOutput;
    type Error = OrganizationPictureObjectDeleteCommandHandlerError;
    type Uow = U;

    fn authorization_plan(
        &self,
        _command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::System,
        ]))
    }

    async fn handle(
        &self,
        _uow: &mut Self::Uow,
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let request = ObjectDeleteRequest::new(
            self.bucket_name.clone(),
            ObjectName::new(command.object_name.value().to_owned())?,
        );
        self.object_deleter.delete(request).await?;

        Ok(CommandHandled::same(OrganizationPictureObjectDeleteOutput))
    }
}
