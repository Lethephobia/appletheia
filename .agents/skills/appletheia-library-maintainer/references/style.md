# Style

Guidelines for repository-wide Rust style, file layout, imports, and source organization.

## File Layout

### DO keep one primary definition per file when practical

Keep a file focused on a single primary `struct`, `enum`, or `trait` so changes stay easy to review. Small `#[cfg(test)]` unit tests may live in the same file when that keeps them close to the implementation.

good:
```rust
pub struct ExampleValue;
```

bad:
```rust
pub struct ExampleValue;
pub struct AnotherValue;
```

### DON'T use `crate::...` or `super::...` directly inside expressions

Import items with `use` and refer to them by name in expressions.

good:
```rust
use crate::value::ExampleValue;

let value = ExampleValue::new();
```

bad:
```rust
let value = crate::value::ExampleValue::new();
```

### PREFER keep related items together when they form a small unit

Use a single module when the types and helpers are meant to change together.

### AVOID sprawling grab-bag modules

Split a module when unrelated concerns start accumulating in the same file.

### CONSIDER splitting a module only when it improves reviewability

Prefer the simplest layout that still makes the public surface easy to understand.

### PREFER feature folders for concepts that grow into several related files

Group a concept into a directory once it needs state, errors, payloads, handlers, and helpers that should evolve together.

### PREFER thin `lib.rs` files that re-export the public surface

Keep crate roots as indexes over submodules instead of burying the API in the root file.

## Visibility

### PREFER `pub(super)` or `pub(crate)` for helpers and fields that do not belong in the public API

Keep internal state and helper functions visible only as far as the surrounding module structure needs.

### DON'T promote internal helpers to `pub` for convenience

Make the public surface reflect the actual contract, not the easiest testing path.

## Source Hygiene

### DO keep application-specific concepts out of the library crates

Keep the reusable crates generic and let downstream applications own business-specific behavior.

### DON'T mix unrelated concerns into a shared utility module

Keep the module boundary aligned with the responsibility of the code.
