# Aggregate Guidelines

Use for aggregate boundaries, command methods, state transitions, and event application.

## Aggregate

### DO define command methods on the aggregate and call `append_event` from them

Keep write-side behavior inside the aggregate boundary.

good:
```rust
pub fn rename(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    self.ensure_opened()?;
    self.append_event(ExampleEventPayload::Renamed { name })
}
```

bad:
```rust
aggregate.append_event(ExampleEventPayload::Renamed { name });
```

### DO build the event payload inside the command method

Construct the payload from the validated command input before you append it.

good:
```rust
pub fn open(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    self.ensure_not_opened()?;
    self.append_event(ExampleEventPayload::Opened {
        id: ExampleId::new(),
        name,
    })
}
```

bad:
```rust
pub fn open(&mut self, event: ExampleEventPayload) -> Result<(), ExampleError> {
    self.append_event(event)
}
```

### DO validate the request before you append an event

Reject invalid requests before any state change is recorded.

good:
```rust
pub fn rename(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    self.ensure_opened()?;
    self.append_event(ExampleEventPayload::Renamed { name })
}
```

bad:
```rust
pub fn rename(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::Renamed { name })
}
```

### DO treat repeated updates to the same state as no-ops when repeating them is harmless

Return `Ok(())` instead of an error for idempotent requests.

good:
```rust
if self.state().is_some_and(|state| state.name == name) {
    return Ok(());
}

self.append_event(ExampleEventPayload::Renamed { name })
```

bad:
```rust
if self.state().is_some_and(|state| state.name == name) {
    return Err(ExampleError::AlreadyRenamed);
}
```

### DO run validation before the no-op check

Do not let a repeated state hide a real validation failure.

good:
```rust
self.ensure_opened()?;

if self.state().is_some_and(|state| state.name == name) {
    return Ok(());
}
```

bad:
```rust
if self.state().is_some_and(|state| state.name == name) {
    return Ok(());
}

self.ensure_opened()?;
```

### DO return an error instead of a no-op for one-shot methods that should only succeed once

Treat repeated create, open, or close calls as misuse.

good:
```rust
pub fn open(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    if self.state().is_some() {
        return Err(ExampleError::AlreadyOpened);
    }

    self.append_event(ExampleEventPayload::Opened {
        id: ExampleId::new(),
        name,
    })
}
```

bad:
```rust
pub fn open(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    if self.state().is_some_and(|state| state.name == name) {
        return Ok(());
    }

    self.append_event(ExampleEventPayload::Opened {
        id: ExampleId::new(),
        name,
    })
}
```

### DO generate the aggregate's own `AggregateId` inside the aggregate's own command method

Keep identity creation within the aggregate boundary.

good:
```rust
pub fn open(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::Opened {
        id: ExampleId::new(),
        name,
    })
}
```

bad:
```rust
pub fn open(&mut self, id: ExampleId, name: ExampleName) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::Opened { id, name })
}
```

### PREFER expose state attributes and computed values through getters

Use read-only accessors when callers need the current state or a derived value.

good:
```rust
pub fn name(&self) -> Result<&ExampleName, ExampleError> {
    Ok(&self.state_required()?.name)
}
```

bad:
```rust
let name = aggregate.state.name.clone();
```

### DON'T give the aggregate any fields other than `core`

Keep aggregate data inside `AggregateCore` and the aggregate state.

bad:
```rust
pub struct ExampleAggregate {
    core: AggregateCore<ExampleState, ExampleEventPayload>,
    name: ExampleName,
}
```

good:
```rust
pub struct ExampleAggregate {
    core: AggregateCore<ExampleState, ExampleEventPayload>,
}
```

### PREFER move validations shared by several command methods into private helper methods

Keep public methods focused on intent and share repeated checks through private helpers.

good:
```rust
fn ensure_opened(&self) -> Result<(), ExampleError> {
    if self.state().is_none() {
        return Err(ExampleError::NotOpened);
    }

    Ok(())
}

pub fn rename(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    self.ensure_opened()?;
    self.append_event(ExampleEventPayload::Renamed { name })
}
```

bad:
```rust
pub fn rename(&mut self, name: ExampleName) -> Result<(), ExampleError> {
    if self.state().is_none() {
        return Err(ExampleError::NotOpened);
    }

    self.append_event(ExampleEventPayload::Renamed { name })
}
```

### DON'T define trigger-only command methods or events on the aggregate that creates another aggregate

Put the creation command on the aggregate that is actually being created.

bad:
```rust
impl Parent {
    pub fn request_open_child(&mut self, name: ChildName) -> Result<(), ParentError> {
        self.append_event(ParentEventPayload::ChildOpenRequested { name })
    }
}

impl Child {
    pub fn open(&mut self, name: ChildName) -> Result<(), ChildError> {
        self.append_event(ChildEventPayload::Opened {
            id: ChildId::new(),
            name,
        })
    }
}
```

good:
```rust
impl Child {
    pub fn open(&mut self, name: ChildName) -> Result<(), ChildError> {
        self.append_event(ChildEventPayload::Opened {
            id: ChildId::new(),
            name,
        })
    }
}
```

### DON'T reference other aggregates directly

Pass aggregate identifiers instead of aggregate instances.

bad:
```rust
pub fn transfer_to(
    &mut self,
    target: ExampleAggregate,
    amount: Money,
) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::Transferred {
        target_id: target.id(),
        amount,
    })
}
```

good:
```rust
pub fn transfer_to(
    &mut self,
    target_id: ExampleId,
    amount: Money,
) -> Result<(), ExampleError> {
    self.append_event(ExampleEventPayload::Transferred { target_id, amount })
}
```

## AggregateApply

### DON'T put validation in `apply`

Keep validation in command methods, not in event replay.

bad:
```rust
fn apply(&mut self, event: ExampleEventPayload) -> Result<(), ExampleError> {
    if self.state_required()?.is_closed() {
        return Err(ExampleError::Closed);
    }

    match event {
        ExampleEventPayload::Renamed { name } => {
            self.state_required_mut()?.name = name;
            Ok(())
        }
        ExampleEventPayload::Opened { id, name } => {
            self.state = Some(ExampleState::new(id, name));
            Ok(())
        }
    }
}
```

good:
```rust
fn apply(&mut self, event: ExampleEventPayload) -> Result<(), ExampleError> {
    match event {
        ExampleEventPayload::Renamed { name } => {
            self.state_required_mut()?.name = name;
            Ok(())
        }
        ExampleEventPayload::Opened { id, name } => {
            self.state = Some(ExampleState::new(id, name));
            Ok(())
        }
    }
}
```

### DON'T skip events that `apply` receives

If an event cannot be applied, return an error so compatibility problems stay visible.

bad:
```rust
fn apply(&mut self, event: ExampleEventPayload) -> Result<(), ExampleError> {
    match event {
        ExampleEventPayload::Renamed { name } => {
            if let Some(state) = self.state_mut() {
                state.name = name;
            }

            Ok(())
        }
        ExampleEventPayload::Opened { id, name } => {
            self.state = Some(ExampleState::new(id, name));
            Ok(())
        }
    }
}
```

good:
```rust
fn apply(&mut self, event: ExampleEventPayload) -> Result<(), ExampleError> {
    match event {
        ExampleEventPayload::Renamed { name } => {
            let state = self.state_required_mut()?;
            state.name = name;
            Ok(())
        }
        ExampleEventPayload::Opened { id, name } => {
            self.state = Some(ExampleState::new(id, name));
            Ok(())
        }
    }
}
```

## AggregateState

### PREFER keep `AggregateState` fields `pub(super)` or `pub(crate)` at most

Limit field visibility to the aggregate module or its parent when possible.

good:
```rust
pub(super) struct ExampleState {
    pub(super) id: ExampleId,
    pub(super) name: ExampleName,
}
```

bad:
```rust
pub struct ExampleState {
    pub id: ExampleId,
    pub name: ExampleName,
}
```

### PREFER provide a constructor when the state has a default value

Use a constructor to capture default state in one place.

good:
```rust
impl ExampleState {
    pub(super) fn new(id: ExampleId, name: ExampleName) -> Self {
        Self {
            id,
            name,
            status: ExampleStatus::Pending,
            description: None,
        }
    }
}
```

bad:
```rust
pub(super) struct ExampleState {
    pub(super) id: ExampleId,
    pub(super) name: ExampleName,
    pub(super) status: ExampleStatus,
    pub(super) description: Option<ExampleDescription>,
}
```

### AVOID define methods other than constructors on `AggregateState`, and keep logic out of it

Keep `AggregateState` as a data container and update it directly from `AggregateApply`.

bad:
```rust
impl ExampleState {
    pub fn rename(&mut self, name: ExampleName) {
        self.name = name;
    }
}
```

good:
```rust
impl AggregateApply<ExampleEventPayload> for ExampleAggregate {
    fn apply(&mut self, event: ExampleEventPayload) -> Result<(), ExampleError> {
        if let ExampleEventPayload::Renamed { name } = event {
            self.state_required_mut()?.name = name;
        }

        Ok(())
    }
}
```

### PREFER define value objects

Prefer domain-specific types over primitive fields when the meaning matters.

good:
```rust
pub(super) struct ExampleState {
    pub(super) name: ExampleName,
    pub(super) balance: Money,
}
```

bad:
```rust
pub(super) struct ExampleState {
    pub(super) name: String,
    pub(super) balance: i64,
}
```

### AVOID use floating-point types such as `f64` in `AggregateState`

Use a fixed-point representation and keep the decimal precision explicit.

bad:
```rust
pub(super) struct ExampleState {
    pub(super) amount: f64,
}
```

good:
```rust
pub(super) struct ExampleState {
    pub(super) amount: i64,
    pub(super) decimals: CurrencyDecimals,
}
```

## EventPayload

### PREFER model `EventPayload` as an enum

Use enum variants to represent distinct facts instead of collapsing them into a struct with optional fields.

good:
```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ExampleEventPayload {
    Opened { id: ExampleId, name: ExampleName },
    DisplayNameChanged { name: ExampleName },
}
```

bad:
```rust
#[derive(Serialize, Deserialize)]
pub struct ExampleEventPayload {
    pub kind: String,
    pub id: Option<ExampleId>,
    pub name: Option<ExampleName>,
}
```

### PREFER adjacently tagged JSON for `EventPayload` serialization

Use `#[serde(tag = "type", content = "data")]` so each variant stays explicit on the wire and payload shapes can evolve without flattening everything into one object.

good:
```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ExampleEventPayload {
    Opened { id: ExampleId, name: ExampleName },
}
```

bad:
```rust
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExampleEventPayload {
    Opened { id: ExampleId, name: ExampleName },
}
```

### DO use past participles for variant names

Make variants read as facts about what already happened.

good:
```rust
pub enum ExampleEventPayload {
    Opened { id: ExampleId, name: ExampleName },
    DisplayNameChanged { name: ExampleName },
}
```

bad:
```rust
pub enum ExampleEventPayload {
    Open { id: ExampleId, name: ExampleName },
    ChangedDisplayName { name: ExampleName },
}
```
