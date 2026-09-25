use core::num::NonZeroU32;

use appletheia::application::authorization::{AuthorizationPlan, PrincipalRequirement};
use appletheia::application::command::CommandHandler;
use appletheia::application::repository::{
    ReferenceIndexLookup, ReferenceIndexLookupPageSize, Repository,
};
use appletheia::application::request_context::RequestContext;
use appletheia::domain::Aggregate;
use banking_ledger_domain::account::{Account, AccountId, AccountOwner, AccountState};
use banking_ledger_domain::owned_account_closure::{
    OwnedAccountClosure, OwnedAccountClosurePageLoadResult,
};

use super::{
    OwnedAccountClosurePageLoadCommand, OwnedAccountClosurePageLoadCommandHandlerError,
    OwnedAccountClosurePageLoadOutput,
};

/// Handles `OwnedAccountClosurePageLoadCommand`.
pub struct OwnedAccountClosurePageLoadCommandHandler<OACR, RIL>
where
    OACR: Repository<OwnedAccountClosure>,
    RIL: ReferenceIndexLookup<Uow = OACR::Uow>,
{
    owned_account_closure_repository: OACR,
    reference_index_lookup: RIL,
}

impl<OACR, RIL> OwnedAccountClosurePageLoadCommandHandler<OACR, RIL>
where
    OACR: Repository<OwnedAccountClosure>,
    RIL: ReferenceIndexLookup<Uow = OACR::Uow>,
{
    pub fn new(owned_account_closure_repository: OACR, reference_index_lookup: RIL) -> Self {
        Self {
            owned_account_closure_repository,
            reference_index_lookup,
        }
    }
}

impl<OACR, RIL> CommandHandler for OwnedAccountClosurePageLoadCommandHandler<OACR, RIL>
where
    OACR: Repository<OwnedAccountClosure>,
    RIL: ReferenceIndexLookup<Uow = OACR::Uow>,
{
    type Command = OwnedAccountClosurePageLoadCommand;
    type Output = OwnedAccountClosurePageLoadOutput;
    type Error = OwnedAccountClosurePageLoadCommandHandlerError;
    type Uow = OACR::Uow;

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
        uow: &mut Self::Uow,
        request_context: &RequestContext,
        command: &Self::Command,
    ) -> Result<Self::Output, Self::Error> {
        let mut owned_account_closure = self
            .owned_account_closure_repository
            .read(uow, command.owned_account_closure_id)
            .await?;

        let owner = owned_account_closure.owner()?;
        let page_size = ReferenceIndexLookupPageSize::new(
            NonZeroU32::new(command.page_size)
                .ok_or(OwnedAccountClosurePageLoadCommandHandlerError::ZeroPageSize)?,
        );

        let page = match owner {
            AccountOwner::User(user_id) => {
                self.reference_index_lookup
                    .find_source_ids::<AccountId, _>(
                        uow,
                        Account::TYPE,
                        AccountState::OWNER_USER_REF,
                        user_id,
                        command.cursor,
                        page_size,
                    )
                    .await?
            }
            AccountOwner::Organization(organization_id) => {
                self.reference_index_lookup
                    .find_source_ids::<AccountId, _>(
                        uow,
                        Account::TYPE,
                        AccountState::OWNER_ORGANIZATION_REF,
                        organization_id,
                        command.cursor,
                        page_size,
                    )
                    .await?
            }
        };

        let result = owned_account_closure.load_page(page.source_ids, page.next_cursor)?;
        self.owned_account_closure_repository
            .save(uow, request_context, &mut owned_account_closure)
            .await?;

        let output = match result {
            OwnedAccountClosurePageLoadResult::Loaded => OwnedAccountClosurePageLoadOutput::Loaded,
            OwnedAccountClosurePageLoadResult::Rejected { reason } => {
                OwnedAccountClosurePageLoadOutput::Rejected { reason }
            }
        };

        Ok(output)
    }
}
