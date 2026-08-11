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

### DO make command outputs own their replay representation

Implement `CommandOutput` directly on every immediate output. Use the output itself as
`ReplayOutput` only when the complete value is safe to persist. When the immediate output contains
credentials, tokens, exchange codes, or other secrets, use a distinct replay-safe type. Keep that
conversion with the output type so every handler return path follows the same policy. Return a
borrowed `CommandReplayOutput` for a self-replaying output and an owned one for a separately built
replay DTO; the command dispatcher serializes that replay value for idempotency storage.

good:
```rust
#[derive(Deserialize, Serialize)]
struct AccountOpenOutput {
    account_id: AccountId,
}

impl CommandOutput for AccountOpenOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
```

good:
```rust
impl CommandOutput for OidcCompleteOutput {
    type ReplayOutput = OidcCompleteReplayOutput;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Owned(self.replay_safe_output())
    }
}
```

bad:
```rust
// A secret-bearing output must not persist and replay itself.
impl CommandOutput for OidcCompleteOutput {
    type ReplayOutput = Self;

    fn replay_output(&self) -> CommandReplayOutput<'_, Self::ReplayOutput> {
        CommandReplayOutput::Borrowed(self)
    }
}
```

### DO classify command outcomes before choosing `Ok` or `Err`

Use the following table to keep domain outcomes, rollback failures, persistence, replay, and
consumer delivery behavior aligned.

| Classification | Typical examples | Handler result and retryability | Persisted data and replay | Transaction | Consumer delivery |
| --- | --- | --- | --- | --- | --- |
| Successful domain outcome | Created, transferred, reserved | `Ok(Output)` | Save domain events when state changed and complete the idempotency record; replay returns the stored output | Commit | Ack |
| Expected persisted business rejection | Insufficient funds, handle already taken, cross-aggregate mismatch | `Ok(Output)` with `Rejected { reason }` | Save a rejected event when the refusal is a domain fact or must drive a saga/projection, then complete the idempotency record | Commit | Ack |
| Expected non-persisted rejection | Invalid externally validated address, unsupported upload content type, policy refusal with no downstream reaction | `Ok(Output)` with `Rejected { reason }` | Save no domain event; complete the idempotency record so replay returns the same rejection | Commit | Ack |
| Non-retryable processing failure | Aggregate invariant violation, required aggregate not found, invalid persisted mapping, impossible application state | `Err`, with `is_retryable() == false` | Save neither pending domain events nor a completed idempotency result; an explicit future submission executes again | Roll back | Ack the current delivery |
| Retryable processing failure | Temporary database, object-storage, network, or application-service failure | `Err`, with `is_retryable() == true` | Save neither pending domain events nor a completed idempotency result; broker redelivery executes again | Roll back | Nack; provider policy may eventually dead-letter |

Treat `Retryability` as the automatic consumer-redelivery decision, not as a prohibition on an
explicit future client submission. Let an outer application error override a lower-level default
when the operation gives the same source error different retry semantics.

good:
```rust
let result = transfer.request(request)?;
transfer_repository
    .save(uow, request_context, &mut transfer)
    .await?;

let output = match result {
    TransferRequestResult::Requested => TransferRequestOutput::Requested { transfer_id },
    TransferRequestResult::Rejected { reason } => {
        TransferRequestOutput::Rejected { transfer_id, reason }
    }
};

Ok(output)
```

bad:
```rust
let result = transfer.request(request)?;
transfer_repository
    .save(uow, request_context, &mut transfer)
    .await?;

if let TransferRequestResult::Rejected { reason } = result {
    // Returning Err rolls back the rejected event and misclassifies a business outcome.
    return Err(TransferRequestCommandHandlerError::Rejected { reason });
}

Ok(TransferRequestOutput::Requested {
    transfer_id,
})
```

### DO persist a rejection only when later behavior needs the fact

Return an expected rejection through the command output without appending an event when no aggregate
state changes and no saga, projection, audit requirement, or later command depends on the refusal.
The completed idempotency record is sufficient to replay the command output. Do not create an empty
or uninitialized aggregate stream solely to persist a rejection reason.

good:
```rust
let address_validation = address_validator.validate(&command.address).await?;
if matches!(address_validation, AddressValidationResult::Invalid) {
    return Ok(RegisterOutput::Rejected {
        registration_id,
        reason: RegisterRejectionReason::InvalidAddress,
    });
}

registration.register(request)?;
registration_repository
    .save(uow, request_context, &mut registration)
    .await?;
```

bad:
```rust
let address_validation = address_validator.validate(&command.address).await?;
if matches!(address_validation, AddressValidationResult::Invalid) {
    // Nothing consumes this event and the aggregate never becomes registered.
    registration.reject_register(request, RegisterRejectionReason::InvalidAddress)?;
    registration_repository
        .save(uow, request_context, &mut registration)
        .await?;

    return Ok(RegisterOutput::Rejected {
        registration_id,
        reason: RegisterRejectionReason::InvalidAddress,
    });
}
```

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

When the aggregate command method appends events and returns a domain result such as `Accepted` or
`Rejected`, save the aggregate and return the result through the command output. Handler-side
validation may return an expected rejection without saving an event when the refusal has no later
domain use. `CommandHandler::Error` is for processing failures that should roll back, not for
expected business outcomes. Implement `Retryability` on the error and use `is_retryable` to control
automatic retry after rollback.

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

Ok(output)
```

bad:
```rust
account.reserve_funds(command.amount)?;
repository.save(uow, request_context, &mut account).await?;

Ok(AccountReserveFundsOutput)
```

### DON'T convert expected domain rejections into handler errors

If a saga or projection must react to a refusal, that refusal must be a persisted domain event.
Returning `Err` rolls back the event write. The command worker negatively acknowledges errors for
which `is_retryable` returns `true` and acknowledges errors for which it returns `false`, so the saga
will never observe the business failure.

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
Ok(output)
```

### DO implement retryability close to the application error that owns it

Implement `Retryability` on reusable repository, authentication, and other application-service
errors when their classification is stable. The error type assigned to `CommandHandler::Error`
must also implement `Retryability`; delegate to source application errors and classify domain errors
at that outer boundary so the domain crate remains independent of application retry policy. An outer
error may override a source error when its operation has different retry semantics.

good:
```rust
impl Retryability for OrganizationCreateCommandHandlerError {
    fn is_retryable(&self) -> bool {
        match self {
            Self::OrganizationRepository(error) => error.is_retryable(),
            Self::Organization(_) => false,
        }
    }
}
```

bad:
```rust
fn repository_error_retryability(error: &RepositoryError<Organization>) -> bool {
    match error {
        RepositoryError::NotFound { .. } => false,
        RepositoryError::EventReader(_) => true,
        // Repeated in every application that uses RepositoryError.
        _ => false,
    }
}
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

Use the handler for lookups that span multiple aggregates or read models. If the failure must be
recorded on the aggregate being commanded, call an aggregate command method that appends a rejection
event and save it. Otherwise, return the rejection without creating an event. Keep `Err` for missing
aggregates, repository failures, and other processing failures that should roll back. Classify
automatic retry through `Retryability`.

good:
```rust
let currency = currency_repository.find_by_id(uow, command.currency_id).await?;
let mut issuance = CurrencyIssuance::new();
let currency_issuance_id = issuance.aggregate_id();
let request = CurrencyIssuanceRequest {
    currency_id: command.currency_id,
    destination_account_id: command.destination_account_id,
    amount: command.amount,
};

if destination_account.currency_id()? != &command.currency_id {
    let reason = CurrencyIssuanceIssueRejectionReason::CurrencyMismatch;
    issuance.reject_issue(request, reason)?;

    currency_issuance_repository
        .save(uow, request_context, &mut issuance)
        .await?;

    return Ok(CurrencyIssueOutput::Rejected {
        currency_issuance_id,
        reason,
    });
}

if !currency.is_active() {
    let reason = CurrencyIssuanceIssueRejectionReason::CurrencyInactive;
    issuance.reject_issue(request, reason)?;

    currency_issuance_repository
        .save(uow, request_context, &mut issuance)
        .await?;

    return Ok(CurrencyIssueOutput::Rejected {
        currency_issuance_id,
        reason,
    });
}

let result = issuance.issue(request)?;

currency_issuance_repository
    .save(uow, request_context, &mut issuance)
    .await?;

let output = match result {
    CurrencyIssuanceIssueResult::Issued => CurrencyIssueOutput::Issued {
        currency_issuance_id,
    },
    CurrencyIssuanceIssueResult::Rejected { reason } => CurrencyIssueOutput::Rejected {
        currency_issuance_id,
        reason,
    },
};

Ok(output)
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
