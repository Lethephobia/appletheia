## Repository Positioning (Important)

This repository (`appletheia` and its related crates) is a **general-purpose library** for building **CQRS / Event Sourcing** applications.

- This repository itself is **not an application**.
- **Do not add application-specific implementations or definitions here** (domain logic, concrete Commands / CommandHandlers, Sagas, Projectors, etc.).
- The **user's application** that depends on this library is responsible for implementing business/domain concepts and handlers.
- This repository provides **interfaces (traits)**, **default implementations**, and supporting infrastructure components to make Event Sourcing development easier.

With that in mind, when proposing designs or changes, do not assume an “app implementation that lives inside this repo”. Prefer **generic abstractions, extensibility, and swap-ability** suitable for a reusable library.
