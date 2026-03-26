# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
