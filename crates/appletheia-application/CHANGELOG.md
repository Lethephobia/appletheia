# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.27.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.26.0...appletheia-application-v0.27.0) - 2026-09-25

### Added

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

### Other

- *(messaging)* [**breaking**] move CloudEvents encoding into Pub/Sub codecs
- *(messaging)* [**breaking**] move topic names into Pub/Sub infrastructure
- *(messaging)* [**breaking**] namespace subscriptions by topic and worker role
- *(messaging)* [**breaking**] restore direct Pub/Sub envelope transport
- *(saga)* [**breaking**] assign consumer group suffixes in workers
- *(messaging)* [**breaking**] use explicit CloudEvent type prefix names
- *(messaging)* clarify CloudEvent dependency names
- *(messaging)* [**breaking**] extract envelope CloudEvent codecs and tighten trait bounds
- [**breaking**] align conversion method names with ownership and fallibility
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
- standardize domain and application re-exports on globs
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

## [0.26.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.25.0...appletheia-application-v0.26.0) - 2026-07-15

### Added

- *(aggregate)* [**breaking**] make aggregate identifiers intrinsic
- *(repository)* [**breaking**] make aggregate reads return not found errors
- *(object-storage)* [**breaking**] add direct object uploader
- *(projection)* [**breaking**] generalize projection consistency waiting

### Fixed

- *(repository)* persist events without aggregate state

### Other

- *(application-oidc)* rename OidcBirthdate full constructor
- *(application)* keep birthdate parsers scoped
- *(application)* [**breaking**] split OIDC birthdate value objects
- *(application)* adjust checksum algorithm string conversion
- *(application)* [**breaking**] simplify read-your-writes consistency
- *(application)* [**breaking**] infer event selector aggregate type

## [0.25.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.24.0...appletheia-application-v0.25.0) - 2026-05-08

### Added

- *(application)* [**breaking**] allow multiple saga start events
- *(repository)* [**breaking**] add paged reference index lookup
- *(repository)* [**breaking**] add aggregate reference indexes

### Other

- *(repository)* align reference lookup cursor naming

## [0.24.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.23.0...appletheia-application-v0.24.0) - 2026-05-06

### Added

- *(repository)* [**breaking**] move relationship updates to save hooks
- *(banking)* project ledger read models
- *(command)* [**breaking**] persist domain rejection outcomes

### Other

- *(saga)* [**breaking**] enforce instance start events
- *(saga)* [**breaking**] restore instance-based workflows

## [0.23.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.22.0...appletheia-application-v0.23.0) - 2026-04-27

### Added

- *(saga)* [**breaking**] support no-command transitions
- *(object-storage)* add object deleter

## [0.22.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.21.0...appletheia-application-v0.22.0) - 2026-04-22

### Other

- *(collections)* [**breaking**] align collection wrappers with domain semantics

## [0.21.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.20.0...appletheia-application-v0.21.0) - 2026-04-21

### Added

- *(object-storage)* [**breaking**] add signed object upload signer

### Other

- Refine OIDC and JWT claim errors
- Add profile picture upload preparation
- [**breaking**] normalize enum serde tagging

## [0.20.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.19.0...appletheia-application-v0.20.0) - 2026-04-13

### Other

- *(saga)* [**breaking**] remove correlation from persistence keys
- *(saga)* [**breaking**] identify runs by trigger event
- *(saga)* [**breaking**] remove handler saga dependencies
- *(saga)* [**breaking**] model saga runs as transitions

## [0.19.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.18.0...appletheia-application-v0.19.0) - 2026-04-09

### Other

- *(saga)* rename SagaAppendCommandError to SagaInstanceError
- *(application)* add relationship builders for projectors
- *(application)* use is_for_aggregate in sagas and projectors
- *(workspace)* fix repository urls

## [0.18.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.17.0...appletheia-application-v0.18.0) - 2026-04-08

### Added

- *(application)* [**breaking**] make request context construction fallible
- *(application)* [**breaking**] move saga reactions into command options
- *(command)* add field patch and user bio

### Other

- *(authorization)* simplify handler authorization plans
- *(authorization)* [**breaking**] tighten subject filtering and owner handling
- *(authorization)* redesign relation model
- *(command)* make patch and context serialization explicit
- *(workspace)* derive default and update sha2

## [0.17.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.16.0...appletheia-application-v0.17.0) - 2026-03-30

### Added

- *(workspace)* add command failure reactions and transfer saga handling

## [0.16.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.15.0...appletheia-application-v0.16.0) - 2026-03-30

### Fixed

- *(saga)* allow terminal instances without state
- *(saga)* skip terminal instances before marking processed

### Other

- *(saga)* simplify default saga runner report handling

## [0.15.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.14.0...appletheia-application-v0.15.0) - 2026-03-29

### Other

- *(workspace)* support correlation read-your-writes targets
- *(workspace)* align descriptors and read-your-writes waiting

## [0.14.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.13.1...appletheia-application-v0.14.0) - 2026-03-26

### Other

- *(command)* remove handler projector dependencies

## [0.13.1](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.13.0...appletheia-application-v0.13.1) - 2026-03-26

### Other

- *(application)* simplify static authorization references

## [0.13.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.12.0...appletheia-application-v0.13.0) - 2026-03-26

### Added

- *(macros)* add command attribute macro

### Other

- *(application)* document application value objects
- *(application)* split projector and saga specs from services

## [0.12.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.11.0...appletheia-application-v0.12.0) - 2026-03-25

### Added

- *(oidc)* [**breaking**] add continuation abstractions and begin result
- *(oidc)* [**breaking**] add begin result and login attempt timestamp value objects

### Other

- *(appletheia-application)* decouple command output from replay output

## [0.11.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.10.1...appletheia-application-v0.11.0) - 2026-03-24

### Added

- *(repository)* add repository lookup by unique value

## [0.10.1](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.10.0...appletheia-application-v0.10.1) - 2026-03-19

### Other

- Revert "chore(release): release crates"
- *(release)* release crates
- *(workspace)* centralize shared dependency versions

## [0.10.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.9.0...appletheia-application-v0.10.0) - 2026-03-18

### Added

- *(projection)* add reset support for processed event stores

## [0.9.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.8.0...appletheia-application-v0.9.0) - 2026-03-18

### Other

- *(oidc)* use bool for verified claims

## [0.8.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.7.0...appletheia-application-v0.8.0) - 2026-03-17

### Added

- *(oidc)* add standard claims and userinfo support

## [0.7.0](https://github.com/Lethephobia/appletheia/compare/appletheia-application-v0.6.0...appletheia-application-v0.7.0) - 2026-03-16

### Added

- *(messaging)* [**breaking**] split pubsub publishers and subscribers
