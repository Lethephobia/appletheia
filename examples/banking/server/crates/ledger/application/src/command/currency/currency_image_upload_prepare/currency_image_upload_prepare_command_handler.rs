use appletheia::application::authorization::{
    AuthorizationPlan, PrincipalRequirement, Relation, RelationshipRequirement,
};
use appletheia::application::command::{CommandHandled, CommandHandler};
use appletheia::application::object_storage::{
    ObjectName, ObjectUploadSignRequest, ObjectUploadSigner,
};
use appletheia::application::repository::Repository;
use appletheia::application::request_context::RequestContext;
use banking_ledger_domain::currency::{Currency, CurrencyImageObjectName, CurrencyImageRef};

use super::{
    CurrencyImageUploadPrepareCommand, CurrencyImageUploadPrepareCommandHandlerConfig,
    CurrencyImageUploadPrepareCommandHandlerError, CurrencyImageUploadPrepareOutput,
};
use crate::authorization::CurrencyUpdaterRelation;

/// Handles `CurrencyImageUploadPrepareCommand`.
pub struct CurrencyImageUploadPrepareCommandHandler<CR, OUS>
where
    CR: Repository<Currency>,
    OUS: ObjectUploadSigner,
{
    currency_repository: CR,
    object_upload_signer: OUS,
    config: CurrencyImageUploadPrepareCommandHandlerConfig,
}

impl<CR, OUS> CurrencyImageUploadPrepareCommandHandler<CR, OUS>
where
    CR: Repository<Currency>,
    OUS: ObjectUploadSigner,
{
    pub fn new(
        currency_repository: CR,
        object_upload_signer: OUS,
        config: CurrencyImageUploadPrepareCommandHandlerConfig,
    ) -> Self {
        Self {
            currency_repository,
            object_upload_signer,
            config,
        }
    }
}

impl<CR, OUS> CommandHandler for CurrencyImageUploadPrepareCommandHandler<CR, OUS>
where
    CR: Repository<Currency>,
    OUS: ObjectUploadSigner,
{
    type Command = CurrencyImageUploadPrepareCommand;
    type Output = CurrencyImageUploadPrepareOutput;
    type ReplayOutput = CurrencyImageUploadPrepareOutput;
    type Error = CurrencyImageUploadPrepareCommandHandlerError;
    type Uow = CR::Uow;

    fn authorization_plan(
        &self,
        command: &Self::Command,
    ) -> Result<AuthorizationPlan, Self::Error> {
        Ok(AuthorizationPlan::OnlyPrincipals(vec![
            PrincipalRequirement::AuthenticatedWithRelationship(RelationshipRequirement::check::<
                Currency,
            >(
                command.currency_id,
                CurrencyUpdaterRelation::REF,
            )),
        ]))
    }

    async fn handle(
        &self,
        uow: &mut Self::Uow,
        _request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<CommandHandled<Self::Output, Self::ReplayOutput>, Self::Error> {
        let Some(currency) = self
            .currency_repository
            .find(uow, command.currency_id)
            .await?
        else {
            return Err(CurrencyImageUploadPrepareCommandHandlerError::CurrencyNotFound);
        };

        if currency.is_removed()? {
            return Err(CurrencyImageUploadPrepareCommandHandlerError::CurrencyRemoved);
        }

        if command.content_length.value() > self.config.max_content_length().value() {
            return Err(CurrencyImageUploadPrepareCommandHandlerError::ContentLengthTooLarge);
        }

        if !self
            .config
            .allowed_content_types()
            .contains(&command.content_type)
        {
            return Err(CurrencyImageUploadPrepareCommandHandlerError::ContentTypeNotAllowed);
        }

        let image_object_name = CurrencyImageObjectName::new(command.currency_id);
        let image = CurrencyImageRef::object_name(image_object_name.clone());
        let object_name = ObjectName::new(image_object_name.value().to_owned())?;
        let request = ObjectUploadSignRequest::new(
            self.config.bucket_name().clone(),
            object_name,
            command.content_type.clone(),
            self.config.expires_in(),
        )
        .with_content_length(command.content_length)
        .with_checksum(command.checksum.clone());
        let signed_upload = self.object_upload_signer.sign(request).await?;
        let output = CurrencyImageUploadPrepareOutput::new(image, signed_upload);

        Ok(CommandHandled::same(output))
    }
}
