# Saga Guidelines

Use for workflow orchestration across aggregates, event-driven command emission, and saga state.

## Saga

### DO treat sagas as workflow coordinators

Use sagas to connect domain events to follow-up commands across aggregate boundaries.

good:
```rust
if let TransferEventPayload::Requested { .. } = transfer_event.payload() {
    instance.append_command(event, &AccountReserveFundsCommand { .. })?;
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
    "Requested" => { /* ... */ }
    _ => {}
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
    instance.append_command(event, &AccountReserveFundsCommand { .. })?;
}
```

### DON'T depend on repositories, read model stores, or relationship stores from a saga

Sagas should stay event-driven and self-contained.
Keep them independent from application services unless the workflow absolutely requires a dedicated integration boundary.

good:
```rust
let transfer_event = event.try_into_domain_event::<Transfer>()?;
```

bad:
```rust
let organization = organization_repository.find_by_id(...).await?;
let members = relationship_store.read_subjects_by_aggregate(...).await?;
let summary = read_model_store.find_by_organization_id(...).await?;
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
*instance.state_mut() = Some(TransferSagaState::new(*id, *from_account_id, *to_account_id, *amount));
instance.append_command(event, &AccountReserveFundsCommand { .. })?;
```

bad:
```rust
command_bus.send(AccountReserveFundsCommand { .. });
```

### DO mark the saga succeeded or failed on terminal events

Use explicit terminal transitions when the workflow completes or aborts.

good:
```rust
match transfer_event.payload() {
    TransferEventPayload::Completed => instance.succeed(),
    TransferEventPayload::Failed => instance.fail(),
    _ => {}
}
```

bad:
```rust
match transfer_event.payload() {
    TransferEventPayload::Completed => {}
    TransferEventPayload::Failed => {}
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
stored business IDs when the subscription and correlation already guarantee the event came from the same workflow.
after the saga start event initialized that state.
Treat later handlers as operating on already-established saga state instead of
re-validating that state field by field.

good:
```rust
instance.append_command(
    event,
    &AccountDepositCommand {
        account_id: state.to_account_id,
        amount: state.amount,
    },
    CommandOptions::default(),
)?;
```

bad:
```rust
if state.status != TransferSagaStatus::Requested {
    return Err(TransferSagaError::UnexpectedStatus);
}

let to_account_id = state
    .to_account_id
    .ok_or(TransferSagaError::MissingState)?;
let amount = state.amount.ok_or(TransferSagaError::MissingState)?;

instance.append_command(
    event,
    &AccountDepositCommand {
        account_id: state.to_account_id,
        amount: state.amount,
    },
    CommandOptions::default(),
)?;
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

instance.append_command(event, &CompleteSetupCommand { .. }, CommandOptions::default())?;
```

## SagaSpec

### DO declare the event subscription explicitly

Keep the saga's trigger set visible and stable.

good:
```rust
const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
    SagaName::new("transfer"),
    Subscription::Only(&[
        EventSelector::new(Transfer::TYPE, TransferEventPayload::REQUESTED),
        EventSelector::new(Account::TYPE, AccountEventPayload::FUNDS_RESERVED),
    ]),
);
```

bad:
```rust
const DESCRIPTOR: SagaDescriptor = SagaDescriptor::new(
    SagaName::new("transfer"),
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
Subscription::Only(&[
    EventSelector::new(OrganizationInvitation::TYPE, OrganizationInvitationEventPayload::ACCEPTED),
    EventSelector::new(OrganizationMembership::TYPE, OrganizationMembershipEventPayload::CREATED),
])
```

bad:
```rust
Subscription::Only(&[
    EventSelector::new(OrganizationInvitation::TYPE, OrganizationInvitationEventPayload::CREATED),
    EventSelector::new(OrganizationInvitation::TYPE, OrganizationInvitationEventPayload::ACCEPTED),
    EventSelector::new(OrganizationInvitation::TYPE, OrganizationInvitationEventPayload::DECLINED),
    EventSelector::new(OrganizationInvitation::TYPE, OrganizationInvitationEventPayload::CANCELED),
    EventSelector::new(OrganizationMembership::TYPE, OrganizationMembershipEventPayload::CREATED),
    EventSelector::new(OrganizationMembership::TYPE, OrganizationMembershipEventPayload::ACTIVATED),
    EventSelector::new(OrganizationMembership::TYPE, OrganizationMembershipEventPayload::INACTIVATED),
    EventSelector::new(OrganizationMembership::TYPE, OrganizationMembershipEventPayload::REMOVED),
])
```

## SagaState

### DO store only the correlation data needed to complete the workflow

Keep saga state compact and focused on in-flight ids.

good:
```rust
pub struct TransferSagaState {
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    pub amount: AccountBalance,
    pub transfer_id: TransferId,
}
```

bad:
```rust
pub struct TransferSagaState {
    pub from_account_balance: Option<AccountBalance>,
    pub to_account_balance: Option<AccountBalance>,
    pub transfer_total: Option<AccountBalance>,
}
```

### PREFER an explicit workflow status or phase in saga state when progress matters

When a saga has multiple meaningful steps, store a compact status enum so the state shows both
the routing ids and how far the workflow has advanced.

Use the status to model saga-local progress transitions, not to mirror aggregate business status.

good:
```rust
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
    Requested,
    FundsReserved,
    Deposited,
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
    pub organization_invitation_id: Option<OrganizationInvitationId>,
    pub organization_id: Option<OrganizationId>,
    pub invitee_id: Option<UserId>,
}
```

bad:
```rust
pub struct OrganizationInvitationSagaState {
    pub organization_name: Option<OrganizationName>,
    pub invitee_username: Option<Username>,
    pub invitation_status: Option<InvitationStatus>,
}
```

### PREFER serializable and compact saga state

Persisted state should be easy to serialize and cheap to restore.

good:
```rust
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
