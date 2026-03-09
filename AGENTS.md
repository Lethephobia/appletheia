## Repository Positioning (Important)

This repository (`appletheia` and its related crates) is a **general-purpose library** for building **CQRS / Event Sourcing** applications.

- This repository itself is **not an application**.
- **Do not add application-specific implementations or definitions here** (domain logic, concrete Commands / CommandHandlers, Sagas, Projectors, etc.).
- The **user's application** that depends on this library is responsible for implementing business/domain concepts and handlers.
- This repository provides **interfaces (traits)**, **default implementations**, and supporting infrastructure components to make Event Sourcing development easier.

With that in mind, when proposing designs or changes, do not assume an “app implementation that lives inside this repo”. Prefer **generic abstractions, extensibility, and swap-ability** suitable for a reusable library.

## Coding Rules (Added)

- As a principle, keep **one primary definition per file** (`trait` / `struct` / `enum`, etc.). Avoid putting multiple primary definitions in a single file.
  - Exception: `#[cfg(test)]` unit tests may live in the same file (preferred).
- Do not reference `crate::...` / `super::...` directly inside expressions; import via `use` and then use the imported names (for readability and stable diffs).
- In unit tests and macro tests, prefer `thiserror::Error` for custom error definitions instead of manually implementing `Display` and `Error`.
- Unit tests in non-macro crates may use proc macros from `appletheia-macros` through `dev-dependencies` when that improves readability for test-only helper types.
- When a unit test directly targets a core trait or value object, prefer a handwritten implementation for the item under test; helper types may still use macros.
- Macro expansion behavior must be tested in `appletheia-macros`; domain/application/infrastructure tests should stay focused on the trait or API being verified.

## Documentation Comments

- Write Rust doc comments (`///`) in **English**.
- Follow common Rust API documentation style:
  - Start with a short summary sentence describing what the item represents or does.
  - Add one or more short paragraphs only when they clarify responsibilities, invariants, or extension points.
  - Prefer describing **behavior and contract** over implementation details.
  - Use inline code formatting for identifiers, types, and literals.
- Keep doc comments concise. Do not add long examples or exhaustive discussion unless they materially improve API usage.

## Git Commit Messages

- When generating or proposing a commit message, follow the commit message convention defined in `CONTRIBUTING.md` (Conventional Commits).
