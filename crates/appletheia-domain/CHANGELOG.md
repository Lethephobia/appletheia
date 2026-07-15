# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0](https://github.com/Lethephobia/appletheia/compare/appletheia-domain-v0.5.0...appletheia-domain-v0.6.0) - 2026-07-15

### Added

- *(aggregate)* [**breaking**] make aggregate identifiers intrinsic
- *(domain)* add aggregate constructor

### Fixed

- *(repository)* persist events without aggregate state

## [0.5.0](https://github.com/Lethephobia/appletheia/compare/appletheia-domain-v0.4.0...appletheia-domain-v0.5.0) - 2026-05-08

### Added

- *(domain)* simplify unique and reference index values
- *(repository)* [**breaking**] add aggregate reference indexes

## [0.4.0](https://github.com/Lethephobia/appletheia/compare/appletheia-domain-v0.3.3...appletheia-domain-v0.4.0) - 2026-05-06

### Other

- *(saga)* [**breaking**] enforce instance start events

## [0.3.3](https://github.com/Lethephobia/appletheia/compare/appletheia-domain-v0.3.2...appletheia-domain-v0.3.3) - 2026-04-21

### Other

- *(snapshot)* simplify getting aggregate id

## [0.3.2](https://github.com/Lethephobia/appletheia/compare/appletheia-domain-v0.3.1...appletheia-domain-v0.3.2) - 2026-04-09

### Other

- *(workspace)* fix repository urls

## [0.3.1](https://github.com/Lethephobia/appletheia/compare/appletheia-domain-v0.3.0...appletheia-domain-v0.3.1) - 2026-03-19

### Fixed

- *(domain)* remove macro test dependency cycle

### Other

- Revert "chore(release): release crates"
- *(release)* release crates
- *(workspace)* centralize shared dependency versions
- *(macros)* derive Display for aggregate IDs
