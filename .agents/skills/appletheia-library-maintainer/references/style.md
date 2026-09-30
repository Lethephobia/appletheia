# Style

Guidelines for repository-wide Rust style, file layout, imports, and source organization.

## File Layout

### DO keep one primary definition per file

Keep a file focused on a single primary `struct`, `enum`, or `trait` so changes stay easy to review.
When adding a new `trait`, `enum`, or `struct`, create a dedicated module file and re-export it
from the parent module. Small `#[cfg(test)]` unit tests may live in the same file when that keeps
them close to the implementation.

**Good**

```rust
// example_value.rs
pub struct ExampleValue;
```

**Bad**

```rust
// example.rs
pub struct ExampleValue;
pub struct AnotherValue;
```

### DON'T use `crate::...` or `super::...` directly inside expressions

Import items with `use` and refer to them by name in expressions.

**Good**

```rust
use crate::value::ExampleValue;

let value = ExampleValue::new();
```

**Bad**

```rust
let value = crate::value::ExampleValue::new();
```

### DON'T use `expect` or `unwrap` in non-test code

Propagate errors or handle them explicitly in library, application, and example code. Reserve `expect` and `unwrap` for tests and fixtures where the failure context is part of the assertion.

**Good**

```rust
let value = MaybeValue::try_from(input).map_err(ValueError::from)?;
```

**Bad**

```rust
let value = MaybeValue::try_from(input).expect("input should be valid");
```

### DON'T shadow local bindings

Give each intermediate value a distinct name that reflects its role. Keep the original binding
available for review instead of redeclaring the same name after a conversion or match.

**Good**

```rust
let begin_attempt = service.begin().await;
let begin_result = match begin_attempt {
    Ok(value) => value,
    Err(operation_error) => {
        let rolled_back_error = uow.rollback_with_operation_error(operation_error).await?;
        return Err(rolled_back_error.into());
    }
};
```

**Bad**

```rust
let begin_result = service.begin().await;
let begin_result = match begin_result {
    Ok(value) => value,
    Err(operation_error) => {
        let operation_error = uow.rollback_with_operation_error(operation_error).await?;
        return Err(operation_error.into());
    }
};
```

### PREFER import concrete domain and application types instead of qualifying them through the crate name

Bring commonly used external types into scope once, then refer to them by bare name in signatures and expressions.

**Good**

```rust
use banking_iam_domain::{Organization, OrganizationId, UserId};
use uuid::Uuid;

let organization = AggregateRef::from_id::<Organization>(organization_id);
let user_id = UserId::new();
let correlation_id = Uuid::now_v7();
```

**Bad**

```rust
let organization = AggregateRef::from_id::<banking_iam_domain::Organization>(organization_id);
let user_id = banking_iam_domain::UserId::new();
let correlation_id = uuid::Uuid::now_v7();
```

### PREFER keep related items together when they form a small unit

Use a single module when the types and helpers are meant to change together.

**Good**

```text
selector.rs: primary selector type, its impl, and focused unit tests
```

**Bad**

```text
selector.rs + selector_helpers.rs + selector_tests.rs for a few tightly coupled lines
```

### PREFER keeping type-specific helpers inside the relevant `impl`

When a small helper only exists to support one type's behavior, keep it as a private associated
function on that type instead of a free function.

**Good**

```rust
impl ExampleSelector {
    pub const fn matches_static(&self, other: &Self) -> bool {
        Self::str_eq(self.name, other.name)
    }

    const fn str_eq(left: &str, right: &str) -> bool {
        // ...
    }
}
```

**Bad**

```rust
impl ExampleSelector {
    pub const fn matches_static(&self, other: &Self) -> bool {
        str_eq(self.name, other.name)
    }
}

const fn str_eq(left: &str, right: &str) -> bool {
    // ...
}
```

### AVOID sprawling grab-bag modules

Split a module when unrelated concerns start accumulating in the same file.

**Good**

```text
messaging/: messaging types
authorization/: authorization types
```

**Bad**

```text
utils.rs: message decoding, authorization policies, and unrelated SQL helpers
```

### CONSIDER splitting a module only when it improves reviewability

Prefer the simplest layout that still makes the public surface easy to understand.

**Good**

```text
Separate independent primary types; keep a short private helper beside its owning impl
```

**Bad**

```text
Extract every private helper into its own module despite having only one caller
```

### PREFER feature folders for concepts that grow into several related files

Group a concept into a directory once it needs state, errors, payloads, handlers, and helpers that should evolve together.

**Good**

```text
saga/
  saga_definition.rs
  saga_definition_builder.rs
  saga_definition_error.rs
```

**Bad**

```text
common/
  saga_definition.rs
  auth_token.rs
  object_storage_error.rs
```

### PREFER thin `lib.rs` files that re-export the public surface

Keep crate roots as indexes over submodules instead of burying the API in the root file.

**Good**

```rust
pub mod event;
pub mod aggregate;
```

**Bad**

```rust
// lib.rs contains the full implementations of every aggregate and event type.
pub struct AggregateCore { /* ... */ }
pub struct EventEnvelope { /* ... */ }
```

## Visibility

### PREFER `pub(super)` or `pub(crate)` for helpers and fields that do not belong in the public API

Keep internal state and helper functions visible only as far as the surrounding module structure needs.

**Good**

```rust
pub struct ExampleState {
    pub(super) value: ExampleValue,
}
```

**Bad**

```rust
pub struct ExampleState {
    pub value: ExampleValue, // Exposes internal representation to downstream crates.
}
```

### DON'T promote internal helpers to `pub` for convenience

Make the public surface reflect the actual contract, not the easiest testing path.

**Good**

```rust
impl ExampleSelector {
    fn normalize(input: &str) -> String {
        input.trim().to_owned()
    }
}
```

**Bad**

```rust
impl ExampleSelector {
    // Public only so an external test can call it.
    pub fn normalize(input: &str) -> String {
        input.trim().to_owned()
    }
}
```

## Source Hygiene

### DO keep application-specific concepts out of the library crates

Keep the reusable crates generic and let downstream applications own business-specific behavior.

**Good**

```text
Library: Repository<A>
Banking application: AccountFundsReserveCommandHandler
```

**Bad**

```text
Library: BankingAccountFundsReserveCommandHandler
```

### DON'T mix unrelated concerns into a shared utility module

Keep the module boundary aligned with the responsibility of the code.

**Good**

```text
Keep token decoding in authentication and message decoding in messaging
```

**Bad**

```text
Place token decoding, account validation, and SQL generation in shared/utils.rs
```
