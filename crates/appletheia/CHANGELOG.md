# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.8.1...appletheia-v0.9.0) - 2026-09-25

### Added

- *(infrastructure)* [**breaking**] make SHA, JWT and HTTP implementations opt-in
- *(infrastructure)* [**breaking**] make AES-GCM opt-in and unify re-exports
- *(infrastructure)* [**breaking**] make PostgreSQL implementations opt-in
- *(infrastructure)* [**breaking**] make Google Cloud implementations opt-in
- *(messaging)* [**breaking**] migrate envelope transport to CloudEvent adapters
- *(messaging)* [**breaking**] support optional types and extension filters in CloudEvent selectors
- *(messaging)* add CloudEvent transport contracts and Pub/Sub implementations
- *(messaging)* add CloudEvent and attribute value objects
- *(watch)* add session registry and shared index contract
- *(authorization)* [**breaking**] derive relationships from registered relations during save
- *(application)* [**breaking**] route terminal command failures to sagas
- *(read-model)* [**breaking**] refactor fragment watch pipeline
- *(command)* [**breaking**] add replay-safe command outputs
- *(application)* [**breaking**] add error retryability contract

### Fixed

- *(infrastructure)* match JWT algorithms to verification key families
- *(infrastructure)* separate command failure dead letters

### Other

- *(messaging)* [**breaking**] move CloudEvents encoding into Pub/Sub codecs
- [**breaking**] align conversion method names with ownership and fallibility
- standardize domain and application re-exports on globs
- *(messaging)* [**breaking**] move topic names into Pub/Sub infrastructure
- *(messaging)* [**breaking**] namespace subscriptions by topic and worker role
- *(messaging)* [**breaking**] restore direct Pub/Sub envelope transport
- *(saga)* [**breaking**] assign consumer group suffixes in workers
- *(messaging)* [**breaking**] use explicit CloudEvent type prefix names
- *(messaging)* clarify CloudEvent dependency names
- *(messaging)* [**breaking**] extract envelope CloudEvent codecs and tighten trait bounds
- *(messaging)* [**breaking**] separate delivery errors from consumer errors
- *(messaging)* [**breaking**] make publisher message an associated type
- *(watch)* [**breaking**] remove watch implementation ahead of redesign
- *(messaging)* [**breaking**] derive ordering keys from publishable messages
- *(outbox)* [**breaking**] separate invalidation outbox identity and dead letters
- *(projection)* [**breaking**] return typed invalidated partitions from projectors
- *(projection)* [**breaking**] align naming with invalidation-based projection
- *(saga)* [**breaking**] require handler errors to accept event envelope errors
- *(saga)* [**breaking**] unify handler errors and validate event names in envelopes
- *(saga)* construct builder routes through existing constructors
- update Alloy, ICU locale, and macro tooling
- *(application)* [**breaking**] define sagas with staged route builders
- *(application)* [**breaking**] make workers reusable
- *(application)* [**breaking**] move saga state type to Saga
- *(application)* [**breaking**] unify saga completion state
- *(application)* clarify saga causative step naming
- *(read-model)* replace fragment change pipeline with invalidation
- *(command)* [**breaking**] remove single-command outbox enqueue
- *(event)* [**breaking**] separate event persistence from outbox enqueue
- *(command)* [**breaking**] move command envelopes to command module
- *(security)* add dependency checks and update JWT crypto backend
- *(infrastructure)* [**breaking**] upgrade jsonwebtoken to v11
- *(infrastructure)* update cloud and utility dependencies
- *(macros)* upgrade syn to v3

## [0.8.1](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.8.0...appletheia-v0.8.1) - 2026-07-15

### Other

- updated the following local packages: appletheia-domain, appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.8.0](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.12...appletheia-v0.8.0) - 2026-05-08

### Added

- *(repository)* [**breaking**] add aggregate reference indexes

## [0.7.12](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.11...appletheia-v0.7.12) - 2026-05-06

### Added

- *(banking-ledger)* add owned account list query

## [0.7.11](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.10...appletheia-v0.7.11) - 2026-04-27

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.7.10](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.9...appletheia-v0.7.10) - 2026-04-22

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.7.9](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.8...appletheia-v0.7.9) - 2026-04-21

### Other

- updated the following local packages: appletheia-domain, appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.7.8](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.7...appletheia-v0.7.8) - 2026-04-13

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.7.7](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.6...appletheia-v0.7.7) - 2026-04-09

### Other

- *(workspace)* fix repository urls

## [0.7.6](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.5...appletheia-v0.7.6) - 2026-04-08

### Other

- *(authorization)* redesign relation model

## [0.7.5](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.4...appletheia-v0.7.5) - 2026-03-30

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.7.4](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.3...appletheia-v0.7.4) - 2026-03-30

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.7.3](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.2...appletheia-v0.7.3) - 2026-03-29

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.7.2](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.1...appletheia-v0.7.2) - 2026-03-26

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.7.1](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.7.0...appletheia-v0.7.1) - 2026-03-26

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.7.0](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.6.8...appletheia-v0.7.0) - 2026-03-26

### Added

- *(macros)* add relations attribute macro
- *(macros)* add command attribute macro

### Other

- *(example)* use command macro in banking commands

## [0.6.8](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.6.7...appletheia-v0.6.8) - 2026-03-25

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure

## [0.6.7](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.6.6...appletheia-v0.6.7) - 2026-03-24

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.6.6](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.6.5...appletheia-v0.6.6) - 2026-03-19

### Other

- updated the following local packages: appletheia-domain, appletheia-application, appletheia-infrastructure, appletheia-macros

## [0.6.5](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.6.4...appletheia-v0.6.5) - 2026-03-18

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure

## [0.6.4](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.6.3...appletheia-v0.6.4) - 2026-03-18

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure

## [0.6.3](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.6.2...appletheia-v0.6.3) - 2026-03-17

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure

## [0.6.2](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.6.1...appletheia-v0.6.2) - 2026-03-16

### Other

- updated the following local packages: appletheia-infrastructure

## [0.6.1](https://github.com/Lethephobia/appletheia/compare/appletheia-v0.6.0...appletheia-v0.6.1) - 2026-03-16

### Other

- updated the following local packages: appletheia-application, appletheia-infrastructure
