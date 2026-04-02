# Documentation

Guidelines for Rust doc comments and API prose.

## Doc Comments

### DO write doc comments in English

Keep public API docs consistent across the repository.

good:
```rust
/// Represents a stable account identifier.
pub struct AccountId;
```

bad:
```rust
/// AccountId.
pub struct AccountId;
```

### DO start doc comments with a short summary sentence

Lead with the behavior or contract the item provides.

### PREFER keeping doc comments short and contract-focused

Describe behavior, invariants, and extension points instead of implementation details.

### DON'T repeat what the signature already says

Use the comment to add context, not to restate the obvious.

### PREFER use inline code formatting for identifiers, types, and literals

Make the API prose easier to scan and less ambiguous.

### AVOID long examples unless they materially improve usage

Move long examples into tests or reference files when they do not fit naturally in the comment.

### CONSIDER code samples for tricky APIs

Use a short example when it makes the contract easier to understand.

## Comment Shape

### DO explain parameters, return values, and errors in prose when needed

Keep the contract readable without requiring the reader to inspect the implementation.

### PREFER noun phrases for types and value objects

Let the doc comment read like a concise description of the item.
