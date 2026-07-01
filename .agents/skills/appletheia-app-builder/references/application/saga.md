# Saga Guidelines

Use for workflow orchestration across aggregates, event-driven command emission, and saga state.

## Saga

### DO treat sagas as workflow coordinators

Use sagas to connect domain events to follow-up commands across aggregate boundaries.

good:
```rust
if let TransferEventPayload::Requested { .. } = transfer_event.payload() {
    instance.append_command(event, &AccountFundsReserveCommand { .. })?;
}
```

bad:
```rust
let mut transfer = repository.find_by_id(uow, command.transfer_id).await?;
transfer.request(command.from_account_id, command.to_account_id, command.amount)?;
repository.save(uow, &transfer).await?;

let mut account = repository.find_by_id(uow, command.from_account_id).await?;
account.reserve_funds(command.amount)?;
repository.save(uow, &account).await?;
```

### DO keep workflow branching explicit

Branch on the aggregate type and payload you actually need.

good:
```rust
if event.is_for_aggregate::<Transfer>() {
    let transfer_event = event.try_into_domain_event::<Transfer>()?;
    // ...
}
```

bad:
```rust
match event.payload().name() {
    "requested" => { /* ... */ }
    _ => {}
}
```

### DO drive compensation and abort paths from domain failure events

Sagas should react to persisted rejection or failure events emitted by aggregates. Do not depend on
command handler `Err` values for workflow failure, because those errors are rolled back and retried
by the command dispatcher and worker.

good:
```rust
match account_event.payload() {
    AccountEventPayload::FundsReserveRejected { .. } => {
        let state = instance.state_required_mut()?;
        state.status = TransferSagaStatus::FailRequested;
        instance.append_command(event, &TransferFailCommand {
            transfer_id: state.transfer_id,
        })?;
    }
    AccountEventPayload::FundsReserved { .. } => {
        // continue the success path
    }
    _ => {}
}
```

bad:
```rust
// command worker failure is not a saga input
if command_failed {
    instance.append_command(event, &TransferFailCommand { transfer_id })?;
}
```

### PREFER enum statuses for linear saga progress

For a single-path workflow, store progress in one status enum and include command-requested states
when they prevent duplicate follow-up commands. Use booleans or sets only for parallel branches or
independent facts that cannot be represented by one current status.

Prefer statuses that describe the command most recently requested by the saga, not the event that
just arrived, when the event immediately triggers another command. Use observed or terminal
statuses only when the saga is intentionally waiting, succeeding, or failing at that point.

good:
```rust
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferSagaStatus {
    #[default]
    Initial,
    FundsReserveRequested,
    DepositRequested,
    ReservedFundsCommitRequested,
    CompleteRequested,
    FailRequested,
    Completed,
    Failed,
}
```

bad:
```rust
pub struct TransferSagaState {
    funds_reserved: bool,
    deposit_requested: bool,
    fail_requested: bool,
    completed: bool,
}
```

### DON'T operate aggregates directly inside a saga

Keep validation and mutation inside command handlers and aggregate command methods.
The saga should only coordinate the workflow by reacting to events and emitting follow-up commands.

bad:
```rust
let mut account = repository.find_by_id(uow, command.account_id).await?;
account.reserve_funds(command.amount)?;
```

good:
```rust
if let TransferEventPayload::Requested { .. } = transfer_event.payload() {
    instance.append_command(event, &AccountFundsReserveCommand { .. })?;
}
```

### DON'T depend on `RequestContext` inside a saga

Use the triggering event payload and saga state as the only workflow inputs.
Avoid reading `event.context.principal`, `event.context.actor`, or any other ambient request metadata in saga logic.
If the workflow needs provenance or issuer information later, put that data into the domain event payload when the event is emitted, not into `RequestContext`.

good:
```rust
let transfer_event = event.try_into_domain_event::<Transfer>()?;
if let TransferEventPayload::Requested { requester_id, .. } = transfer_event.payload() {
    // derive the next command from event data
}
```

bad:
```rust
let actor = &event.context.actor;
let principal = &event.context.principal;
```

### PREFER a saga per workflow

Give each orchestration flow its own saga even when several flows are similar.

good:
```rust
TransferSaga
OrganizationInvitationSaga
OrganizationJoinRequestSaga
```

bad:
```rust
WorkflowSaga
```

### DO use `SagaInstance` to carry state, queued commands, and terminal status

Let the saga implementation use the instance as the single place for in-flight workflow bookkeeping.

good:
```rust
*instance.state_mut() = Some(TransferSagaState::new(
    *id,
    *from_account_id,
    *to_account_id,
    *amount,
));

instance.append_command(
    event,
    &AccountFundsReserveCommand {
        account_id: *from_account_id,
        amount: *amount,
    },
)?;
```

bad:
```rust
command_bus.send(AccountFundsReserveCommand { .. });
```

### DO use `append_command_with_options` only when command options differ from defaults

Use `append_command` for the normal path. Keep explicit options visible only when the saga needs
non-default consistency or other command options.

good:
```rust
instance.append_command(event, &TransferCompleteCommand { transfer_id })?;
```

good:
```rust
instance.append_command_with_options(
    event,
    &TransferCompleteCommand { transfer_id },
    CommandOptions {
        consistency: CommandConsistency::Eventual,
    },
)?;
```

bad:
```rust
instance.append_command_with_options(
    event,
    &TransferCompleteCommand { transfer_id },
    CommandOptions::default(),
)?;
```

### DO mark the saga succeeded or failed on terminal events

Use explicit terminal transitions when the workflow completes or aborts.

good:
```rust
match transfer_event.payload() {
    TransferEventPayload::Completed => {
        instance.state_required_mut()?.status = TransferSagaStatus::Completed;
        instance.succeed();
    }
    TransferEventPayload::Failed { .. } => {
        instance.state_required_mut()?.status = TransferSagaStatus::Failed;
        instance.fail();
    }
    _ => {}
}
```

bad:
```rust
match transfer_event.payload() {
    TransferEventPayload::Completed => {}
    TransferEventPayload::Failed { .. } => {}
    _ => {}
}
```

### DON'T emit follow-up commands after the saga is terminal

Terminal workflows should not keep appending commands.

good:
```rust
instance.succeed();
```

bad:
```rust
instance.succeed();
instance.append_command(event, &AnotherCommand { .. })?;
```

### DON'T add redundant transition validation for strictly ordered saga steps

Commands emitted within one correlation are processed in append order. When the next event can only
arrive after the prior command completed, do not add defensive "previous status must be X" checks
or repeat completeness validation for data the saga already fixed at startup. For the same reason,
do not add extra checks to prove that a follow-up event "really belongs" to the saga by comparing
stored business IDs when the subscription and correlation already guarantee the event came from the
same workflow.

good:
```rust
let state = instance.state_required_mut()?;
let to_account_id = state.to_account_id;
let amount = state.amount;
state.status = TransferSagaStatus::DepositRequested;

instance.append_command(
    event,
    &AccountDepositCommand {
        account_id: to_account_id,
        amount,
    },
)?;
```

bad:
```rust
if state.status != TransferSagaStatus::FundsReserveRequested {
    return Err(TransferSagaError::UnexpectedStatus);
}

let transfer_id = state.transfer_id.ok_or(TransferSagaError::IncompleteState)?;
let from_account_id = state.from_account_id.ok_or(TransferSagaError::IncompleteState)?;
```

### DO use state checks as readiness tracking only for parallel branches

When a saga fans out multiple commands and must wait for all their events, track readiness in saga
state and no-op until every required branch is complete. Treat this as workflow progress tracking,
not as an error condition.

good:
```rust
state.profile_ready = true;
if !state.settings_ready {
    return Ok(());
}

instance.append_command(event, &CompleteSetupCommand { setup_id: state.setup_id })?;
```

## SagaSpec

### DO declare the event subscription explicitly

Keep the saga's trigger set visible and stable.

good:
```rust
const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
    SagaName::new("transfer"),
    SagaStartEvents::new(&[EventSelector::new::<Transfer>(
        TransferEventPayload::REQUESTED,
    )]),
    Subscription::AnyOf(&[
        EventSelector::new::<Transfer>(TransferEventPayload::REQUESTED),
        EventSelector::new::<Account>(AccountEventPayload::FUNDS_RESERVED),
    ]),
);
```

bad:
```rust
const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
    SagaName::new("transfer"),
    SagaStartEvents::new(&[EventSelector::new::<Transfer>(
        TransferEventPayload::REQUESTED,
    )]),
    Subscription::All,
);
```

### DO keep the saga name business-oriented

Name the saga after the workflow or aggregate family, not after one transient step.

good:
```rust
SagaName::new("organization_invitation")
```

bad:
```rust
SagaName::new("invitation_accepted")
```

### PREFER narrow subscriptions

Subscribe to the exact events the saga consumes.

good:
```rust
Subscription::AnyOf(&[
    EventSelector::new::<OrganizationInvitation>(OrganizationInvitationEventPayload::ACCEPTED),
    EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANTED),
    EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANT_REJECTED),
])
```

bad:
```rust
Subscription::AnyOf(&[
    EventSelector::new::<OrganizationInvitation>(OrganizationInvitationEventPayload::ISSUED),
    EventSelector::new::<OrganizationInvitation>(OrganizationInvitationEventPayload::ACCEPTED),
    EventSelector::new::<OrganizationInvitation>(OrganizationInvitationEventPayload::DECLINED),
    EventSelector::new::<OrganizationInvitation>(OrganizationInvitationEventPayload::CANCELED),
    EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANTED),
    EventSelector::new::<User>(UserEventPayload::ORGANIZATION_MEMBERSHIP_GRANT_REJECTED),
    EventSelector::new::<User>(UserEventPayload::EMAIL_CHANGED),
])
```

## SagaState

### DO store only the correlation data needed to complete the workflow

Keep saga state compact and focused on in-flight ids.

good:
```rust
pub struct TransferSagaState {
    pub transfer_id: TransferId,
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
}
```

bad:
```rust
pub struct TransferSagaState {
    pub from_account_balance: Option<CurrencyAmount>,
    pub to_account_balance: Option<CurrencyAmount>,
    pub transfer_total: Option<CurrencyAmount>,
}
```

### PREFER an explicit workflow status or phase in saga state when progress matters

When a saga has multiple meaningful steps, store a compact status enum so the state shows both
the routing ids and how far the workflow has advanced.

Use the status to model saga-local progress transitions, not to mirror aggregate business status.

good:
```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSagaState {
    pub transfer_id: TransferId,
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: CurrencyAmount,
    pub status: TransferSagaStatus,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferSagaStatus {
    #[default]
    Initial,
    FundsReserveRequested,
    DepositRequested,
    ReservedFundsCommitRequested,
    CompleteRequested,
    FailRequested,
    Completed,
    Failed,
}
```

bad:
```rust
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSagaState {
    pub transfer_id: Option<TransferId>,
    pub from_account_id: Option<AccountId>,
    pub to_account_id: Option<AccountId>,
    pub amount: Option<CurrencyAmount>,
}
```

### DON'T duplicate domain state in saga state

Store ids and routing hints, not a second copy of the business aggregate state.

good:
```rust
pub struct OrganizationInvitationSagaState {
    pub organization_invitation_id: OrganizationInvitationId,
}
```

bad:
```rust
pub struct OrganizationInvitationSagaState {
    pub organization_name: Option<OrganizationName>,
    pub invitee_username: Option<Username>,
    pub invitation_status: Option<OrganizationInvitationStatus>,
}
```

### PREFER serializable and compact saga state

Persisted state should be easy to serialize and cheap to restore.

good:
```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExampleSagaState {
    pub example_id: ExampleId,
}
```

bad:
```rust
pub struct ExampleSagaState {
    pub repository: ExampleRepository,
}
```
