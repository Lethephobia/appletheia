use std::marker::PhantomData;

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::object_storage::{
    ObjectBucketName, ObjectDeleteRequest, ObjectDeleter, ObjectName,
};
use appletheia::application::request_context::RequestContext;
use appletheia::application::unit_of_work::UnitOfWork;

use super::{
    UserPictureObjectDeleteCommand, UserPictureObjectDeleteCommandHandlerError,
    UserPictureObjectDeleteOutput,
};

/// Handles `UserPictureObjectDeleteCommand`.
pub struct UserPictureObjectDeleteCommandHandler<OD, U>
where
    OD: ObjectDeleter,
    U: UnitOfWork + Sync,
{
    object_deleter: OD,
    bucket_name: ObjectBucketName,
    _uow: PhantomData<U>,
}

impl<OD, U> UserPictureObjectDeleteCommandHandler<OD, U>
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

impl<OD, U> CommandHandler for UserPictureObjectDeleteCommandHandler<OD, U>
where
    OD: ObjectDeleter,
    U: UnitOfWork + Sync,
{
    type Command = UserPictureObjectDeleteCommand;
    type Output = UserPictureObjectDeleteOutput;
    type ReplayOutput = UserPictureObjectDeleteOutput;
    type Error = UserPictureObjectDeleteCommandHandlerError;
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

        Ok(CommandHandled::same(UserPictureObjectDeleteOutput))
    }
}
