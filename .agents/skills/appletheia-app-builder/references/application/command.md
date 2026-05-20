# Command Guidelines

Use for command payloads, command handlers, authorization, validation, and orchestration.

## Command

### DO keep command input minimal

Include only the data that is necessary to express the intent.

good:
```rust
pub struct OrganizationRemoveCommand {
    pub organization_id: OrganizationId,
}
```

bad:
```rust
pub struct OrganizationRemoveCommand {
    pub organization_id: OrganizationId,
    pub organization_name: OrganizationName,
    pub organization_handle: OrganizationHandle,
}
```

## CommandHandler

### DO load the aggregate, invoke its command method, and save the result

Keep state transitions inside the aggregate boundary.

good:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.change_name(command.name)?;
repository.save(uow, &organization).await?;
```

bad:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.state_mut().name = command.name;
repository.save(uow, &organization).await?;
```

### DO treat domain rejections as successful command handling

When the aggregate command method returns a domain result such as `Accepted` or `Rejected`, save the
aggregate and return the result through the command output. `CommandHandler::Error` is for processing
failures that should roll back and retry, not for expected business outcomes.

good:
```rust
let result = account.reserve_funds(command.amount)?;
repository.save(uow, request_context, &mut account).await?;
let output = match result {
    AccountReserveFundsResult::Reserved => AccountReserveFundsOutput::Reserved,
    AccountReserveFundsResult::Rejected { reason } => {
        AccountReserveFundsOutput::Rejected { reason }
    }
};

Ok(CommandHandled::same(output))
```

bad:
```rust
account.reserve_funds(command.amount)?;
repository.save(uow, request_context, &mut account).await?;

Ok(CommandHandled::same(AccountReserveFundsOutput))
```

### DON'T convert expected domain rejections into handler errors

If a saga or projection must react to a refusal, that refusal must be a persisted domain event.
Returning `Err` rolls back the event write and lets the command worker retry or dead-letter the
message, so the saga will never observe the business failure.

bad:
```rust
if account.available_balance()? < command.amount {
    return Err(AccountReserveFundsCommandHandlerError::InsufficientAvailableBalance);
}
```

good:
```rust
let result = account.reserve_funds(command.amount)?;
repository.save(uow, request_context, &mut account).await?;
let output = match result {
    AccountReserveFundsResult::Reserved => AccountReserveFundsOutput::Reserved,
    AccountReserveFundsResult::Rejected { reason } => {
        AccountReserveFundsOutput::Rejected { reason }
    }
};
Ok(CommandHandled::same(output))
```

### DON'T touch `RequestContext.actor` in command handlers

The default command dispatcher already authorizes commands with `principal`.
Use `actor` only when a workflow explicitly needs provenance or persistence context, not for routine authorization decisions.

good:
```rust
let _principal = request_context.principal.clone();
```

bad:
```rust
let actor = &request_context.actor;
```

### DON'T mutate aggregate state directly in the handler

The handler should orchestrate, not reimplement domain logic.

bad:
```rust
let mut account = repository.find_by_id(uow, command.account_id).await?;
account.state_mut().name = command.name;
```

good:
```rust
let mut account = repository.find_by_id(uow, command.account_id).await?;
account.rename(command.name)?;
```

### DO keep cross-aggregate validation in the handler when the rule cannot live inside one aggregate

Use the handler for lookups that span multiple aggregates or read models. If the failure can be
recorded on the aggregate being commanded, call an aggregate command method that appends a rejection
event and save it. Keep `Err` for missing aggregates, repository failures, and other processing
failures that should roll back and retry.

good:
```rust
let currency = currency_repository.find_by_id(uow, command.currency_id).await?;
let mut issuance = CurrencyIssuance::default();

let result = if destination_account.currency_id()? != &command.currency_id {
    let reason = CurrencyIssuanceIssueRejectionReason::CurrencyMismatch;
    let currency_issuance_id = issuance.reject_issue(
        command.currency_id,
        command.destination_account_id,
        command.amount,
        reason,
    )?;
    CurrencyIssuanceIssueResult::Rejected {
        currency_issuance_id,
        reason,
    }
} else if !currency.is_active() {
    let reason = CurrencyIssuanceIssueRejectionReason::CurrencyInactive;
    let currency_issuance_id = issuance.reject_issue(
        command.currency_id,
        command.destination_account_id,
        command.amount,
        reason,
    )?;
    CurrencyIssuanceIssueResult::Rejected {
        currency_issuance_id,
        reason,
    }
} else {
    issuance.issue(
        command.currency_id,
        command.destination_account_id,
        command.amount,
    )?
};

currency_issuance_repository
    .save(uow, request_context, &mut issuance)
    .await?;

let output = match result {
    CurrencyIssuanceIssueResult::Issued {
        currency_issuance_id,
    } => CurrencyIssuanceIssueOutput::Issued {
        currency_issuance_id,
    },
    CurrencyIssuanceIssueResult::Rejected {
        currency_issuance_id,
        reason,
    } => CurrencyIssuanceIssueOutput::Rejected {
        currency_issuance_id,
        reason,
    },
};

Ok(CommandHandled::same(output))
```

good:
```rust
let account = account_repository.find_by_id(uow, command.account_id).await?;
let source = source_repository.find_by_id(uow, command.source_id).await?;
```

bad:
```rust
let currency = currency_repository.find_by_id(uow, command.currency_id).await?;
let mut issuance = currency_issuance_repository.find_by_id(uow, command.currency_issuance_id).await?;

if !currency.is_active() {
    return Err(CurrencyIssuanceIssueCommandHandlerError::CurrencyInactive);
}

let result = issuance.issue(command.amount)?;
```

### DON'T duplicate aggregate-owned validation in the handler

If the aggregate command method already enforces a rule, let the aggregate own that failure path.
Reserve handler-side checks for rules that need other aggregates or read models.

good:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.change_name(command.name)?;
```

bad:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
if organization.is_removed() {
    return Err(OrganizationChangeNameCommandHandlerError::Removed);
}

organization.change_name(command.name)?;
```

### DON'T orchestrate multi-aggregate workflows directly in the handler

Use a saga when one command needs to emit follow-up commands for another aggregate.

bad:
```rust
let mut invitation = invitation_repository.find_by_id(uow, command.invitation_id).await?;
invitation.accept()?;

let mut membership = membership_repository.find_by_id(uow, command.membership_id).await?;
membership.create()?;
```

good:
```rust
let mut invitation = invitation_repository.find_by_id(uow, command.invitation_id).await?;
invitation.accept()?;
```

### DON'T depend on read model stores or relationship stores in command handlers

Command handlers should work through aggregate repositories and domain methods.
If a workflow needs read model data or relationship graph queries, move that concern to a separate query path or workflow service.

good:
```rust
let mut organization = organization_repository.find_by_id(uow, command.organization_id).await?;
organization.change_name(command.name)?;
```

bad:
```rust
let members = relationship_store.read_subjects_by_aggregate(...).await?;
let summary = read_model_store.find_by_organization_id(...).await?;
```

### DO map non-outcome domain errors into handler errors

Return application-specific errors from the handler boundary when the aggregate reports an invalid
operation or invariant failure. Do not use this for expected business rejections that should be
persisted as events.

good:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.change_handle(command.handle)?;
```

bad:
```rust
let mut organization = repository.find_by_id(uow, command.organization_id).await?;
organization.change_handle(command.handle).unwrap();
```

### PREFER handlers to return outputs from persisted ids or resulting state

Return what the caller needs to continue, not extra read-model data.

good:
```rust
Ok(OrganizationRemoveOutput {
    organization_id: command.organization_id,
})
```

bad:
```rust
Ok(OrganizationRemoveOutput {
    organization: repository.find_by_id(uow, command.organization_id).await?,
})
```

### PREFER one unit of work per handler

Keep the transaction boundary aligned with the command boundary unless a workflow explicitly needs more.

good:
```rust
let mut uow = repository.begin().await?;
// load -> authorize -> mutate -> save
uow.commit().await?;
```

bad:
```rust
let mut uow1 = repository.begin().await?;
let mut uow2 = repository.begin().await?;
```

### DON'T hide one-shot domain failures in the handler

If the aggregate rejects a repeated create, open, approve, or accept call, let that failure surface.

good:
```rust
organization.remove()?;
```

bad:
```rust
if organization.is_removed() {
    return Ok(());
}

organization.remove()?;
```
