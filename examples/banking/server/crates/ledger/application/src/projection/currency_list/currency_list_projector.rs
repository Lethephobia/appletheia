use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::{CurrencyListProjectorError, CurrencyListProjectorSpec};
use crate::read_model::{CurrencyListItemStatus, CurrencyListWriter};

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
        if event.is_for_aggregate::<User>() {
            let domain_event = event.try_into_domain_event::<User>()?;
            let user_id = domain_event.aggregate_id();

            match domain_event.payload() {
                UserEventPayload::Registered { .. } => {
                    self.writer
                        .upsert_owner_user(uow, user_id, event.event_sequence, event.occurred_at)
                        .await?;
                }
                UserEventPayload::UsernameChanged { username } => {
                    self.writer
                        .update_owner_user_username(
                            uow,
                            user_id,
                            username.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                UserEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_owner_user_display_name(
                            uow,
                            user_id,
                            display_name.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                UserEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_owner_user_picture(
                            uow,
                            user_id,
                            picture.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                UserEventPayload::Removed => {
                    self.writer
                        .delete_owner_user(uow, user_id, event.event_sequence, event.occurred_at)
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
                | UserEventPayload::Inactivated
                | UserEventPayload::DeactivateRejected { .. }
                | UserEventPayload::RemoveRejected { .. } => {}
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
                            organization_id,
                            handle.clone(),
                            display_name.clone(),
                            picture.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                OrganizationEventPayload::HandleChanged { handle } => {
                    self.writer
                        .update_owner_organization_handle(
                            uow,
                            organization_id,
                            handle.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                OrganizationEventPayload::DisplayNameChanged { display_name } => {
                    self.writer
                        .update_owner_organization_display_name(
                            uow,
                            organization_id,
                            display_name.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                OrganizationEventPayload::PictureChanged { picture, .. } => {
                    self.writer
                        .update_owner_organization_picture(
                            uow,
                            organization_id,
                            picture.clone(),
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                OrganizationEventPayload::Removed => {
                    self.writer
                        .delete_owner_organization(
                            uow,
                            organization_id,
                            event.event_sequence,
                            event.occurred_at,
                        )
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
                ..
            } => {
                self.writer
                    .upsert_currency(
                        uow,
                        currency_id,
                        *owner,
                        symbol.clone(),
                        name.clone(),
                        *decimals,
                        CurrencyAmount::zero(),
                        CurrencyListItemStatus::Active,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferred { owner } => {
                self.writer
                    .update_currency_owner(
                        uow,
                        currency_id,
                        *owner,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::SymbolChanged { symbol } => {
                self.writer
                    .update_currency_symbol(
                        uow,
                        currency_id,
                        symbol.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::NameChanged { name } => {
                self.writer
                    .update_currency_name(
                        uow,
                        currency_id,
                        name.clone(),
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::SupplyIncreased { amount } => {
                self.writer
                    .increase_currency_supply(
                        uow,
                        currency_id,
                        *amount,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::SupplyDecreased { amount } => {
                self.writer
                    .decrease_currency_supply(
                        uow,
                        currency_id,
                        *amount,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::Activated => {
                self.writer
                    .update_currency_status(
                        uow,
                        currency_id,
                        CurrencyListItemStatus::Active,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::Deactivated => {
                self.writer
                    .update_currency_status(
                        uow,
                        currency_id,
                        CurrencyListItemStatus::Inactive,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::Removed => {
                self.writer
                    .delete_currency(uow, currency_id, event.event_sequence, event.occurred_at)
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferRejected { .. }
            | CurrencyEventPayload::SymbolChangeRejected { .. }
            | CurrencyEventPayload::NameChangeRejected { .. }
            | CurrencyEventPayload::SupplyIncreaseRejected { .. }
            | CurrencyEventPayload::SupplyDecreaseRejected { .. }
            | CurrencyEventPayload::ActivateRejected { .. }
            | CurrencyEventPayload::DeactivateRejected { .. }
            | CurrencyEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
