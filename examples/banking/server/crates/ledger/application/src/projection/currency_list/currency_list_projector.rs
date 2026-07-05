use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};
use banking_shared_kernel_application::read_model::ReadModelEventContext;

use super::{CurrencyListProjectorError, CurrencyListProjectorSpec};
use crate::read_model::{
    CurrencyListCurrencyUpsert, CurrencyListItemStatus, CurrencyListOwnerOrganizationUpsert,
    CurrencyListOwnerUserUpsert, CurrencyListWriter,
};

/// Projects currency events into currency list read models.
pub struct CurrencyListProjector<W>
where
    W: CurrencyListWriter,
{
    writer: W,
}

impl<W> CurrencyListProjector<W>
where
    W: CurrencyListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for CurrencyListProjector<W>
where
    W: CurrencyListWriter,
{
    type Spec = CurrencyListProjectorSpec;
    type Uow = W::Uow;
    type Error = CurrencyListProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        let event_context = ReadModelEventContext::from(event);
        if event.is_for_aggregate::<User>() {
            let domain_event = event.try_into_domain_event::<User>()?;
            let user_id = domain_event.aggregate_id();

            match domain_event.payload() {
                UserEventPayload::Registered { .. } => {
                    self.writer
                        .upsert_owner_user(
                            uow,
                            event_context,
                            CurrencyListOwnerUserUpsert {
                                id: user_id,
                                username: None,
                                display_name: None,
                                picture: None,
                            },
                        )
                        .await?;
                }
                UserEventPayload::UsernameChanged { username } => {
                    self.writer
                        .update_owner_user_username(uow, event_context, user_id, username.clone())
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_owner_user_display_name(
                            uow,
                            event_context,
                            user_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_owner_user_picture(uow, event_context, user_id, picture.clone())
                        .await?;
                }
                UserEventPayload::Removed => {
                    self.writer
                        .delete_owner_user(uow, event_context, user_id)
                        .await?;
                }
                UserEventPayload::IdentityLinked { .. }
                | UserEventPayload::IdentityLinkRejected { .. }
                | UserEventPayload::IdentityEmailChanged { .. }
                | UserEventPayload::IdentityEmailChangeRejected { .. }
                | UserEventPayload::BioChanged { .. }
                | UserEventPayload::BioChangeRejected { .. }
                | UserEventPayload::UsernameChangeRejected { .. }
                | UserEventPayload::DisplayNameChangeRejected { .. }
                | UserEventPayload::PictureChangeRejected { .. }
                | UserEventPayload::Activated
                | UserEventPayload::ActivateRejected { .. }
                | UserEventPayload::Deactivated
                | UserEventPayload::DeactivateRejected { .. }
                | UserEventPayload::RemoveRejected { .. }
                | UserEventPayload::OrganizationMembershipGranted { .. }
                | UserEventPayload::OrganizationMembershipGrantRejected { .. }
                | UserEventPayload::OrganizationMembershipRolesChanged { .. }
                | UserEventPayload::OrganizationMembershipRolesChangeRejected { .. }
                | UserEventPayload::OrganizationMembershipRemoved { .. }
                | UserEventPayload::OrganizationMembershipRemoveRejected { .. } => {}
            }

            return Ok(());
        }

        if event.is_for_aggregate::<Organization>() {
            let domain_event = event.try_into_domain_event::<Organization>()?;
            let organization_id = domain_event.aggregate_id();

            match domain_event.payload() {
                OrganizationEventPayload::Created {
                    handle,
                    display_name,
                    picture,
                    ..
                } => {
                    self.writer
                        .upsert_owner_organization(
                            uow,
                            event_context,
                            CurrencyListOwnerOrganizationUpsert {
                                id: organization_id,
                                handle: handle.clone(),
                                display_name: display_name.clone(),
                                picture: picture.clone(),
                            },
                        )
                        .await?;
                }
                OrganizationEventPayload::HandleChanged { handle } => {
                    self.writer
                        .update_owner_organization_handle(
                            uow,
                            event_context,
                            organization_id,
                            handle.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_owner_organization_display_name(
                            uow,
                            event_context,
                            organization_id,
                            display_name.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_owner_organization_picture(
                            uow,
                            event_context,
                            organization_id,
                            picture.clone(),
                        )
                        .await?;
                }
                OrganizationEventPayload::Removed => {
                    self.writer
                        .delete_owner_organization(uow, event_context, organization_id)
                        .await?;
                }
                OrganizationEventPayload::OwnershipTransferred { .. }
                | OrganizationEventPayload::OwnershipTransferRejected { .. }
                | OrganizationEventPayload::HandleChangeRejected { .. }
                | OrganizationEventPayload::DisplayNameChangeRejected { .. }
                | OrganizationEventPayload::DescriptionChanged { .. }
                | OrganizationEventPayload::DescriptionChangeRejected { .. }
                | OrganizationEventPayload::WebsiteUrlChanged { .. }
                | OrganizationEventPayload::WebsiteUrlChangeRejected { .. }
                | OrganizationEventPayload::PictureChangeRejected { .. }
                | OrganizationEventPayload::RemoveRejected { .. } => {}
            }

            return Ok(());
        }

        let domain_event = event.try_into_domain_event::<Currency>()?;
        let currency_id = domain_event.aggregate_id();

        match domain_event.payload() {
            CurrencyEventPayload::Defined {
                owner,
                symbol,
                name,
                decimals,
                description,
                image,
                ..
            } => {
                self.writer
                    .upsert_currency(
                        uow,
                        event_context,
                        CurrencyListCurrencyUpsert {
                            id: currency_id,
                            owner: *owner,
                            symbol: symbol.clone(),
                            name: name.clone(),
                            decimals: *decimals,
                            description: description.clone(),
                            image: image.clone(),
                            mint_account_address: None,
                            supply: CurrencyAmount::zero(),
                            status: CurrencyListItemStatus::Provisioning,
                        },
                    )
                    .await?;
            }
            CurrencyEventPayload::Provisioned { mint_account } => {
                self.writer
                    .update_mint_account_address(
                        uow,
                        event_context,
                        currency_id,
                        mint_account.mint_account_address().clone(),
                    )
                    .await?;
                self.writer
                    .update_currency_status(
                        uow,
                        event_context,
                        currency_id,
                        CurrencyListItemStatus::Active,
                    )
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferred { owner } => {
                self.writer
                    .update_currency_owner(uow, event_context, currency_id, *owner)
                    .await?;
            }
            CurrencyEventPayload::SymbolChanged { symbol } => {
                self.writer
                    .update_currency_symbol(uow, event_context, currency_id, symbol.clone())
                    .await?;
            }
            CurrencyEventPayload::NameChanged { name } => {
                self.writer
                    .update_currency_name(uow, event_context, currency_id, name.clone())
                    .await?;
            }
            CurrencyEventPayload::DescriptionChanged { description } => {
                self.writer
                    .update_currency_description(
                        uow,
                        event_context,
                        currency_id,
                        description.clone(),
                    )
                    .await?;
            }
            CurrencyEventPayload::ImageChanged { image, .. } => {
                self.writer
                    .update_currency_image(uow, event_context, currency_id, image.clone())
                    .await?;
            }
            CurrencyEventPayload::SupplyCommitted { amount } => {
                self.writer
                    .increase_currency_supply(uow, event_context, currency_id, *amount)
                    .await?;
            }
            CurrencyEventPayload::Activated => {
                self.writer
                    .update_currency_status(
                        uow,
                        event_context,
                        currency_id,
                        CurrencyListItemStatus::Active,
                    )
                    .await?;
            }
            CurrencyEventPayload::Deactivated => {
                self.writer
                    .update_currency_status(
                        uow,
                        event_context,
                        currency_id,
                        CurrencyListItemStatus::Inactive,
                    )
                    .await?;
            }
            CurrencyEventPayload::Removed => {
                self.writer
                    .delete_currency(uow, event_context, currency_id)
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferRejected { .. }
            | CurrencyEventPayload::ProvisionRejected { .. }
            | CurrencyEventPayload::SymbolChangeRejected { .. }
            | CurrencyEventPayload::NameChangeRejected { .. }
            | CurrencyEventPayload::DescriptionChangeRejected { .. }
            | CurrencyEventPayload::ImageChangeRejected { .. }
            | CurrencyEventPayload::MintMetadataSynced
            | CurrencyEventPayload::MintMetadataSyncRejected { .. }
            | CurrencyEventPayload::SupplyReserved { .. }
            | CurrencyEventPayload::SupplyReserveRejected { .. }
            | CurrencyEventPayload::MintSupplySynced { .. }
            | CurrencyEventPayload::SupplyCommitRejected { .. }
            | CurrencyEventPayload::SupplyReleased { .. }
            | CurrencyEventPayload::SupplyReleaseRejected { .. }
            | CurrencyEventPayload::ActivateRejected { .. }
            | CurrencyEventPayload::DeactivateRejected { .. }
            | CurrencyEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
