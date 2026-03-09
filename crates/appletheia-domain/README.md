# Appletheia Domain

> **The Application of Truth**\
> Remembering is the architecture of meaning.

Appletheia Domain provides the core domain-layer building blocks for
event-sourced and CQRS-oriented systems.

It defines the primary contracts and value objects used to model aggregates,
events, and snapshots, while keeping application-specific behavior in user
code.

## Get Started

You can use `appletheia-domain` directly for the core traits and value objects,
and optionally combine it with `appletheia-macros` to reduce boilerplate.

```toml
[dependencies]
appletheia-domain = "0.2"
appletheia-macros = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
uuid = { version = "1", features = ["serde", "v7"] }
```

```rust
use thiserror::Error;
use uuid::Uuid;

use appletheia_domain::{
    Aggregate, AggregateApply, AggregateCore, AggregateError, AggregateId, AggregateState,
    EventPayload,
};
use appletheia_macros::{aggregate, aggregate_id, aggregate_state, event_payload};

#[derive(Debug, Error)]
enum CounterIdError {
    #[error("invalid counter id")]
    Invalid,
}

#[aggregate_id(error = CounterIdError)]
struct CounterId(Uuid);

#[derive(Debug, Error)]
enum CounterStateError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[aggregate_state(error = CounterStateError)]
struct CounterState {
    id: CounterId,
    value: i32,
}

#[derive(Debug, Error)]
enum CounterEventPayloadError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[event_payload(error = CounterEventPayloadError)]
enum CounterEventPayload {
    Created { id: CounterId },
    Incremented { amount: i32 },
}

#[derive(Debug, Error)]
enum CounterError {
    #[error(transparent)]
    Aggregate(#[from] AggregateError<CounterId>),
}

#[aggregate(type = "counter", error = CounterError, core = "core")]
struct Counter {
    core: AggregateCore<CounterState, CounterEventPayload>,
}

impl AggregateApply<CounterEventPayload, CounterError> for Counter {
    fn apply(&mut self, payload: &CounterEventPayload) -> Result<(), CounterError> {
        match payload {
            CounterEventPayload::Created { id } => {
                self.set_state(Some(CounterState { id: *id, value: 0 }));
            }
            CounterEventPayload::Incremented { amount } => {
                self.state_required_mut()?.value += amount;
            }
        }

        Ok(())
    }
}
```

With this setup, the aggregate can append new domain events, replay persisted
events, and materialize snapshots while `appletheia-domain` keeps the core
contracts explicit.

## License

This project is licensed under the [MIT License](./LICENSE).
