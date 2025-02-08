# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.3](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.2...bevy_mod_scripting_core-v0.9.3) - 2025-02-08

### Added

- add static scripts which do not need to be attached to entities to be run (#253)
- add recipient for specific language (#250)

## [0.9.2](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.1...bevy_mod_scripting_core-v0.9.2) - 2025-02-08

### Added

- make `extractors` module non-public (#251)

### Fixed

- add missing extensions in the asset loader (#254)

## [0.9.0-alpha.9](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.0-alpha.8...bevy_mod_scripting_core-v0.9.0-alpha.9) - 2025-01-28

### Fixed

- prevent allocation and component ID ranges from overlapping (#230)

## [0.9.0-alpha.8](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.0-alpha.7...bevy_mod_scripting_core-v0.9.0-alpha.8) - 2025-01-27

### Added

- Add `functions` script method, and create function info scaffolding (#228)
- Call custom `get` and `set` functions on the type when indexing. (#226)
- Add `optional` arguments to script functions (#225)
- re-implement rhai again (#222)
- Add world.with_or_insert_component_mut() (#223)

## [0.9.0-alpha.7](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.0-alpha.6...bevy_mod_scripting_core-v0.9.0-alpha.7) - 2025-01-20

### Added

- [**breaking**] Remove `WorldCallbackAccess` & Combine context args for dynamic functions into one `FunctionCallContext` (#219)
- Add component `upsert` function (#218)

## [0.9.0-alpha.6](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.0-alpha.5...bevy_mod_scripting_core-v0.9.0-alpha.6) - 2025-01-19

### Added

- Don't panic! (#216)

## [0.9.0-alpha.3](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.0-alpha.2...bevy_mod_scripting_core-v0.9.0-alpha.3) - 2025-01-14

### Added

- Implement global namespace registration (#202)
- Improvements to BMS in multi-language context (#194)
- make script contexts public (#193)

## [0.9.0-alpha.2](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.0-alpha.1...bevy_mod_scripting_core-v0.9.0-alpha.2) - 2025-01-05

### Added

- complete plugin re-write

### Other

- ditch alpha pre-releases ([#162](https://github.com/makspll/bevy_mod_scripting/pull/162))

## [0.8.0-alpha.2](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.8.0-alpha.1...bevy_mod_scripting_core-v0.8.0-alpha.2) - 2024-12-03

### Other

- Bump bevy & bevy console ([#153](https://github.com/makspll/bevy_mod_scripting/pull/153))

## [0.8.0-alpha.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.8.0-alpha.0...bevy_mod_scripting_core-v0.8.0-alpha.1) - 2024-11-10

### Other

- update Cargo.toml dependencies

## [0.7.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.6.0...bevy_mod_scripting_core-v0.7.0) - 2024-11-03

### Other

- Send ScriptErrorEvent when load fails. ([#125](https://github.com/makspll/bevy_mod_scripting/pull/125))
- Migrate to bevy 0.14 ([#127](https://github.com/makspll/bevy_mod_scripting/pull/127))
- update metadata
