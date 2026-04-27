# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
