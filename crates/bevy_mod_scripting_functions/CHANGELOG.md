# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.9](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.8...bevy_mod_scripting_functions-v0.9.9) - 2025-02-28

### Added

- Add `GlobalNamespace::system_builder`, `World::add_system` and allow dynamic system creation (#335)
- create global `map` for allowing empty hashmap construction (#329)

### Fixed

- remove `map` global and instead allow hashmap `FromScript` from list of tuples (#332)

## [0.9.7](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.6...bevy_mod_scripting_functions-v0.9.7) - 2025-02-23

### Added

- add `construct` global for constructing arbitrary types & `Union` type (#302)

### Fixed

- remove `reflect_functions` and `file_watcher` flags from bevy dependency (#316)

## [0.9.4](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.3...bevy_mod_scripting_functions-v0.9.4) - 2025-02-12

### Added

- refactor generated bindings to use new derive macro (#268)
- refactor core bindings to use new derive macro (#267)

### Fixed

- don't use `new_unregistered` for most of core bindings macros (#270)

## [0.9.0-alpha.8](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.0-alpha.7...bevy_mod_scripting_functions-v0.9.0-alpha.8) - 2025-01-27

### Added

- Add `functions` script method, and create function info scaffolding (#228)

## [0.9.0-alpha.7](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.0-alpha.6...bevy_mod_scripting_functions-v0.9.0-alpha.7) - 2025-01-20

### Added

- [**breaking**] Remove `WorldCallbackAccess` & Combine context args for dynamic functions into one `FunctionCallContext` (#219)
- Add component `upsert` function (#218)

## [0.9.0-alpha.6](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.0-alpha.5...bevy_mod_scripting_functions-v0.9.0-alpha.6) - 2025-01-19

### Added

- Don't panic! (#216)

## [0.9.0-alpha.5](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.0-alpha.4...bevy_mod_scripting_functions-v0.9.0-alpha.5) - 2025-01-19

### Fixed

- Fix missing functions in codegen (#210)

## [0.9.0-alpha.3](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.0-alpha.2...bevy_mod_scripting_functions-v0.9.0-alpha.3) - 2025-01-14

### Added

- Implement global namespace registration (#202)
- Improvements to BMS in multi-language context (#194)

## [0.9.0-alpha.2](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.0-alpha.1...bevy_mod_scripting_functions-v0.9.0-alpha.2) - 2025-01-05

### Added

- complete plugin re-write
