# Documentation

Guidelines for Rust doc comments and API prose.

## Doc Comments

### DO write doc comments in English

Keep public API docs consistent across the repository.

**Good**

```rust
/// Represents a stable account identifier.
pub struct AccountId;
```

**Bad**

```rust
/// アカウントを識別するIDです。
pub struct AccountId;
```

### DO start doc comments with a short summary sentence

Lead with the behavior or contract the item provides.

**Good**

```rust
/// Returns whether the selector matches the event.
pub fn matches(&self, event: &EventEnvelope) -> bool { /* ... */ }
```

**Bad**

```rust
/// First, the implementation creates an iterator and compares some fields.
pub fn matches(&self, event: &EventEnvelope) -> bool { /* ... */ }
```

### PREFER keeping doc comments short and contract-focused

Describe behavior, invariants, and extension points instead of implementation details.

**Good**

```rust
/// Returns an error if the value is empty.
pub fn try_from_string(value: String) -> Result<Self, ValueError> { /* ... */ }
```

**Bad**

```rust
/// Uses an if statement, then calls is_empty, then constructs the error variant.
pub fn try_from_string(value: String) -> Result<Self, ValueError> { /* ... */ }
```

### DON'T repeat what the signature already says

Use the comment to add context, not to restate the obvious.

**Good**

```rust
/// Returns the available amount after subtracting reservations.
pub fn available_balance(&self) -> Result<CurrencyAmount, AccountError> { /* ... */ }
```

**Bad**

```rust
/// Returns Result<CurrencyAmount, AccountError>.
pub fn available_balance(&self) -> Result<CurrencyAmount, AccountError> { /* ... */ }
```

### PREFER use inline code formatting for identifiers, types, and literals

Make the API prose easier to scan and less ambiguous.

**Good**

```rust
/// Returns `None` when no route matches.
```

**Bad**

```rust
/// Returns None when no route matches.
```

### AVOID long examples unless they materially improve usage

Move long examples into tests or reference files when they do not fit naturally in the comment.

**Good**

```text
Doc comment: a focused call demonstrating the API
Example crate: full application wiring
```

**Bad**

```text
Doc comment: full application startup and dependency wiring before the one relevant call
```

### CONSIDER code samples for tricky APIs

Use a short example when it makes the contract easier to understand.

**Good**

```text
Show a continuation route with its outgoing step and causative step labelled
```

**Bad**

```text
Describe two different step arguments as "the step" without showing their roles
```

## Comment Shape

### DO explain parameters, return values, and errors in prose when needed

Keep the contract readable without requiring the reader to inspect the implementation.

**Good**

```rust
/// Parses a nonempty name, returning `NameError::Empty` for empty input.
```

**Bad**

```rust
/// Parses the input. May fail.
```

### PREFER noun phrases for types and value objects

Let the doc comment read like a concise description of the item.

**Good**

```rust
/// The stable identifier of a saga instance.
pub struct SagaInstanceId(Uuid);
```

**Bad**

```rust
/// Creates a saga instance.
pub struct SagaInstanceId(Uuid);
```
