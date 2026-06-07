use appletheia::application::event::EventEnvelope;
use appletheia::application::projection::Projector;
use banking_iam_domain::{Organization, OrganizationEventPayload, User, UserEventPayload};
use banking_ledger_domain::account::{Account, AccountEventPayload};
use banking_ledger_domain::core::CurrencyAmount;
use banking_ledger_domain::currency::{Currency, CurrencyEventPayload};

use super::{OwnedAccountListProjectorError, OwnedAccountListProjectorSpec};
use crate::read_model::{
    OwnedAccountListAccountUpsert, OwnedAccountListCurrencyUpsert, OwnedAccountListItemStatus,
    OwnedAccountListOwnerOrganizationUpsert, OwnedAccountListOwnerUserUpsert,
    OwnedAccountListWriter, ReadModelEventContext,
};

/// Projects account and currency events into owned account list read models.
pub struct OwnedAccountListProjector<W>
where
    W: OwnedAccountListWriter,
{
    writer: W,
}

impl<W> OwnedAccountListProjector<W>
where
    W: OwnedAccountListWriter,
{
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W> Projector for OwnedAccountListProjector<W>
where
    W: OwnedAccountListWriter,
{
    type Spec = OwnedAccountListProjectorSpec;
    type Uow = W::Uow;
    type Error = OwnedAccountListProjectorError;

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
                            OwnedAccountListOwnerUserUpsert {
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
                            OwnedAccountListOwnerOrganizationUpsert {
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

        if event.is_for_aggregate::<Account>() {
            let domain_event = event.try_into_domain_event::<Account>()?;
            let account_id = domain_event.aggregate_id();

            match domain_event.payload() {
                AccountEventPayload::Opened {
                    owner,
                    name,
                    currency_id,
                    ..
                } => {
                    self.writer
                        .upsert_account(
                            uow,
                            event_context,
                            OwnedAccountListAccountUpsert {
                                id: account_id,
                                owner: *owner,
                                name: name.clone(),
                                currency_id: *currency_id,
                                balance: CurrencyAmount::zero(),
                                reserved_balance: CurrencyAmount::zero(),
                                status: OwnedAccountListItemStatus::Active,
                            },
                        )
                        .await?;
                }
                AccountEventPayload::OwnershipTransferred { owner } => {
                    self.writer
                        .update_account_owner(uow, event_context, account_id, *owner)
                        .await?;
                }
                AccountEventPayload::NameChanged { name } => {
                    self.writer
                        .update_account_name(uow, event_context, account_id, name.clone())
                        .await?;
                }
                AccountEventPayload::Deposited { amount } => {
                    self.writer
                        .increase_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::Withdrawn { amount } => {
                    self.writer
                        .decrease_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::FundsReserved { amount } => {
                    self.writer
                        .reserve_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::ReservedFundsReleased { amount } => {
                    self.writer
                        .release_reserved_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::ReservedFundsCommitted { amount } => {
                    self.writer
                        .commit_reserved_balance(uow, event_context, account_id, *amount)
                        .await?;
                }
                AccountEventPayload::Frozen => {
                    self.writer
                        .update_account_status(
                            uow,
                            event_context,
                            account_id,
                            OwnedAccountListItemStatus::Frozen,
                        )
                        .await?;
                }
                AccountEventPayload::Thawed => {
                    self.writer
                        .update_account_status(
                            uow,
                            event_context,
                            account_id,
                            OwnedAccountListItemStatus::Active,
                        )
                        .await?;
                }
                AccountEventPayload::Closed => {
                    self.writer
                        .delete_account(uow, event_context, account_id)
                        .await?;
                }
                AccountEventPayload::OwnershipTransferRejected { .. }
                | AccountEventPayload::NameChangeRejected { .. }
                | AccountEventPayload::DepositRejected { .. }
                | AccountEventPayload::WithdrawRejected { .. }
                | AccountEventPayload::FundsReserveRejected { .. }
                | AccountEventPayload::ReservedFundsReleaseRejected { .. }
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
                        event_context,
                        OwnedAccountListCurrencyUpsert {
                            id: currency_id,
                            symbol: symbol.clone(),
                            name: name.clone(),
                            decimals: *decimals,
                        },
                    )
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
            CurrencyEventPayload::Removed => {
                self.writer
                    .delete_currency(uow, event_context, currency_id)
                    .await?;
            }
            CurrencyEventPayload::OwnershipTransferred { .. }
            | CurrencyEventPayload::OwnershipTransferRejected { .. }
            | CurrencyEventPayload::SymbolChangeRejected { .. }
            | CurrencyEventPayload::NameChangeRejected { .. }
            | CurrencyEventPayload::DescriptionChanged { .. }
            | CurrencyEventPayload::DescriptionChangeRejected { .. }
            | CurrencyEventPayload::ImageChanged { .. }
            | CurrencyEventPayload::ImageChangeRejected { .. }
            | CurrencyEventPayload::MintAccountMetadataSynced
            | CurrencyEventPayload::MintAccountMetadataSyncRejected { .. }
            | CurrencyEventPayload::Provisioned { .. }
            | CurrencyEventPayload::ProvisionRejected { .. }
            | CurrencyEventPayload::MintSupplySynced { .. }
            | CurrencyEventPayload::SupplyReserved { .. }
            | CurrencyEventPayload::SupplyReserveRejected { .. }
            | CurrencyEventPayload::SupplyCommitted { .. }
            | CurrencyEventPayload::SupplyCommitRejected { .. }
            | CurrencyEventPayload::SupplyReleased { .. }
            | CurrencyEventPayload::SupplyReleaseRejected { .. }
            | CurrencyEventPayload::Activated
            | CurrencyEventPayload::ActivateRejected { .. }
            | CurrencyEventPayload::Deactivated
            | CurrencyEventPayload::DeactivateRejected { .. }
            | CurrencyEventPayload::RemoveRejected { .. } => {}
        }

        Ok(())
    }
}
