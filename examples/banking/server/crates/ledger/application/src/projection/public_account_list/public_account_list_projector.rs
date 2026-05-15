use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::{PublicAccountListProjectorError, PublicAccountListProjectorSpec};
use crate::read_model::{
    PublicAccountListAccountUpsert, PublicAccountListCurrencyUpsert, PublicAccountListItemStatus,
    PublicAccountListOwnerOrganizationUpsert, PublicAccountListOwnerUserUpsert,
    PublicAccountListWriter,
};

/// Projects account and currency events into public account list read models.
pub struct PublicAccountListProjector<W>
where
    W: PublicAccountListWriter,
{
    writer: W,
}

impl<W> PublicAccountListProjector<W>
where
    W: PublicAccountListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for PublicAccountListProjector<W>
where
    W: PublicAccountListWriter,
{
    type Spec = PublicAccountListProjectorSpec;
    type Uow = W::Uow;
    type Error = PublicAccountListProjectorError;

    async fn project(&self, uow: &mut Self::Uow, event: &EventEnvelope) -> Result<(), Self::Error> {
        if event.is_for_aggregate::<User>() {
            let domain_event = event.try_into_domain_event::<User>()?;
            let user_id = domain_event.aggregate_id();

            match domain_event.payload() {
                UserEventPayload::Registered {
                    username,
                    display_name,
                    picture,
                    ..
                } => {
                    self.writer
                        .upsert_owner_user(
                            uow,
                            PublicAccountListOwnerUserUpsert {
                                id: user_id,
                                username: username.clone(),
                                display_name: display_name.clone(),
                                picture: picture.clone(),
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
                        )
                        .await?;
                }
                UserEventPayload::UsernameChanged { username } => {
                    self.writer
                        .update_owner_user_username(
                            uow,
                            user_id,
                            username.clone(),
                            event.event_id,
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
                            event.event_id,
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
                            event.event_id,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                UserEventPayload::Removed => {
                    self.writer
                        .delete_owner_user(
                            uow,
                            user_id,
                            event.event_id,
                            event.event_sequence,
                            event.occurred_at,
                        )
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
                            PublicAccountListOwnerOrganizationUpsert {
                                id: organization_id,
                                handle: handle.clone(),
                                display_name: display_name.clone(),
                                picture: picture.clone(),
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
                        )
                        .await?;
                }
                OrganizationEventPayload::HandleChanged { handle } => {
                    self.writer
                        .update_owner_organization_handle(
                            uow,
                            organization_id,
                            handle.clone(),
                            event.event_id,
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
                            event.event_id,
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
                            event.event_id,
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
                            event.event_id,
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

        if event.is_for_aggregate::<Account>() {
            let domain_event = event.try_into_domain_event::<Account>()?;
            let account_id = domain_event.aggregate_id();

            match domain_event.payload() {
                AccountEventPayload::Opened {
                    owner,
                    currency_id,
                    status,
                    ..
                } => {
                    self.writer
                        .upsert_account(
                            uow,
                            PublicAccountListAccountUpsert {
                                id: account_id,
                                owner: *owner,
                                currency_id: *currency_id,
                                status: PublicAccountListItemStatus::try_from(*status)?,
                                event_id: event.event_id,
                                event_sequence: event.event_sequence,
                                occurred_at: event.occurred_at,
                            },
                        )
                        .await?;
                }
                AccountEventPayload::OwnershipTransferred { owner } => {
                    self.writer
                        .update_account_owner(
                            uow,
                            account_id,
                            *owner,
                            event.event_id,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Frozen => {
                    self.writer
                        .update_account_status(
                            uow,
                            account_id,
                            PublicAccountListItemStatus::Frozen,
                            event.event_id,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Thawed => {
                    self.writer
                        .update_account_status(
                            uow,
                            account_id,
                            PublicAccountListItemStatus::Active,
                            event.event_id,
                            event.event_sequence,
                            event.occurred_at,
                        )
                        .await?;
                }
                AccountEventPayload::Closed => {
                    self.writer
                        .delete_account(uow, account_id, event.event_id, event.event_sequence)
                        .await?;
                }
                AccountEventPayload::OwnershipTransferRejected { .. }
                | AccountEventPayload::NameChanged { .. }
                | AccountEventPayload::NameChangeRejected { .. }
                | AccountEventPayload::Deposited { .. }
                | AccountEventPayload::DepositRejected { .. }
                | AccountEventPayload::Withdrawn { .. }
                | AccountEventPayload::WithdrawRejected { .. }
                | AccountEventPayload::FundsReserved { .. }
                | AccountEventPayload::FundsReserveRejected { .. }
                | AccountEventPayload::ReservedFundsReleased { .. }
                | AccountEventPayload::ReservedFundsReleaseRejected { .. }
                | AccountEventPayload::ReservedFundsCommitted { .. }
                | AccountEventPayload::ReservedFundsCommitRejected { .. }
                | AccountEventPayload::FreezeRejected { .. }
                | AccountEventPayload::ThawRejected { .. }
                | AccountEventPayload::CloseRejected { .. } => {}
            }

            return Ok(());
        }

        let domain_event = event.try_into_domain_event::<Currency>()?;
        let currency_id = domain_event.aggregate_id();

        match domain_event.payload() {
            CurrencyEventPayload::Defined {
                symbol,
                name,
                decimals,
                ..
            } => {
                self.writer
                    .upsert_currency(
                        uow,
                        PublicAccountListCurrencyUpsert {
                            id: currency_id,
                            symbol: symbol.clone(),
                            name: name.clone(),
                            decimals: *decimals,
                            event_id: event.event_id,
                            event_sequence: event.event_sequence,
                            occurred_at: event.occurred_at,
                        },
                    )
                    .await?;
            }
            CurrencyEventPayload::SymbolChanged { symbol } => {
                self.writer
                    .update_currency_symbol(
                        uow,
                        currency_id,
                        symbol.clone(),
                        event.event_id,
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
                        event.event_id,
                        event.event_sequence,
                        event.occurred_at,
                    )
                    .await?;
            }
            CurrencyEventPayload::Removed => {
                self.writer
                    .delete_currency(uow, currency_id, event.event_id, event.event_sequence)
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferred { .. }
            | CurrencyEventPayload::OwnershipTransferRejected { .. }
            | CurrencyEventPayload::SymbolChangeRejected { .. }
            | CurrencyEventPayload::NameChangeRejected { .. }
            | CurrencyEventPayload::SupplyIncreased { .. }
            | CurrencyEventPayload::SupplyIncreaseRejected { .. }
            | CurrencyEventPayload::SupplyDecreased { .. }
            | CurrencyEventPayload::SupplyDecreaseRejected { .. }
            | CurrencyEventPayload::Activated
            | CurrencyEventPayload::ActivateRejected { .. }
            | CurrencyEventPayload::Deactivated
            | CurrencyEventPayload::DeactivateRejected { .. }
            | CurrencyEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
