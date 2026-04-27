# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.0](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.13.0...appletheia-infrastructure-v0.14.0) - 2026-04-27

### Added

- *(saga)* [**breaking**] support no-command transitions
- *(object-storage)* add object deleter

## [0.13.0](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.12.0...appletheia-infrastructure-v0.13.0) - 2026-04-22

### Other

- *(collections)* [**breaking**] align collection wrappers with domain semantics

## [0.12.0](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.11.0...appletheia-infrastructure-v0.12.0) - 2026-04-21

### Added

- *(object-storage)* [**breaking**] add signed object upload signer

### Other

- Refine OIDC and JWT claim errors

## [0.11.0](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.10.1...appletheia-infrastructure-v0.11.0) - 2026-04-13

### Other

- *(saga)* [**breaking**] remove correlation from persistence keys
- *(saga)* [**breaking**] identify runs by trigger event
- *(saga)* [**breaking**] model saga runs as transitions

## [0.10.1](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.10.0...appletheia-infrastructure-v0.10.1) - 2026-04-09

### Other

- *(workspace)* fix repository urls

## [0.10.0](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.9.2...appletheia-infrastructure-v0.10.0) - 2026-04-08

### Added

- *(application)* [**breaking**] move saga reactions into command options

### Other

- *(authorization)* [**breaking**] tighten subject filtering and owner handling
- *(authorization)* redesign relation model

## [0.9.2](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.9.1...appletheia-infrastructure-v0.9.2) - 2026-03-30

### Other

- updated the following local packages: appletheia-application

## [0.9.1](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.9.0...appletheia-infrastructure-v0.9.1) - 2026-03-30

### Fixed

- *(saga)* allow terminal instances without state

## [0.9.0](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.8.3...appletheia-infrastructure-v0.9.0) - 2026-03-29

### Other

- *(workspace)* support correlation read-your-writes targets
- *(workspace)* align descriptors and read-your-writes waiting

## [0.8.3](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.8.2...appletheia-infrastructure-v0.8.3) - 2026-03-26

### Other

- updated the following local packages: appletheia-application

## [0.8.2](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.8.1...appletheia-infrastructure-v0.8.2) - 2026-03-26

### Other

- updated the following local packages: appletheia-application

## [0.8.1](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.8.0...appletheia-infrastructure-v0.8.1) - 2026-03-26

### Other

- updated the following local packages: appletheia-application

## [0.8.0](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.7.6...appletheia-infrastructure-v0.8.0) - 2026-03-25

### Added

- *(oidc)* [**breaking**] add continuation abstractions and begin result
- *(oidc)* [**breaking**] add begin result and login attempt timestamp value objects

## [0.7.6](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.7.5...appletheia-infrastructure-v0.7.6) - 2026-03-24

### Added

- *(repository)* add repository lookup by unique value

## [0.7.5](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.7.4...appletheia-infrastructure-v0.7.5) - 2026-03-19

### Other

- Revert "chore(release): release crates"
- *(release)* release crates
- *(workspace)* centralize shared dependency versions

## [0.7.4](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.7.3...appletheia-infrastructure-v0.7.4) - 2026-03-18

### Added

- *(projection)* add reset support for processed event stores

## [0.7.3](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.7.2...appletheia-infrastructure-v0.7.3) - 2026-03-18

### Other

- *(oidc)* use bool for verified claims

## [0.7.2](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.7.1...appletheia-infrastructure-v0.7.2) - 2026-03-17

### Added

- *(oidc)* add standard claims and userinfo support

## [0.7.1](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.7.0...appletheia-infrastructure-v0.7.1) - 2026-03-16

### Other

- *(infrastructure)* upgrade reqwest and remove legacy pubsub topics

## [0.7.0](https://github.com/Lethephobia/appletheia/compare/appletheia-infrastructure-v0.6.0...appletheia-infrastructure-v0.7.0) - 2026-03-16

### Added

- *(messaging)* [**breaking**] split pubsub publishers and subscribers
