# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0-alpha.8](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.9.0-alpha.7...bevy_mod_scripting_lua-v0.9.0-alpha.8) - 2025-01-27

### Added

- add `ScriptValue::Map` and create appropriate conversions in lua and rhai (#229)
- Add `functions` script method, and create function info scaffolding (#228)
- Call custom `get` and `set` functions on the type when indexing. (#226)
- re-implement rhai again (#222)

## [0.9.0-alpha.7](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.9.0-alpha.6...bevy_mod_scripting_lua-v0.9.0-alpha.7) - 2025-01-20

### Added

- [**breaking**] Remove `WorldCallbackAccess` & Combine context args for dynamic functions into one `FunctionCallContext` (#219)
- Add component `upsert` function (#218)

## [0.9.0-alpha.6](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.9.0-alpha.5...bevy_mod_scripting_lua-v0.9.0-alpha.6) - 2025-01-19

### Added

- Don't panic! (#216)

## [0.9.0-alpha.3](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.9.0-alpha.2...bevy_mod_scripting_lua-v0.9.0-alpha.3) - 2025-01-14

### Added

- Implement global namespace registration (#202)
- Improvements to BMS in multi-language context (#194)

## [0.8.0-alpha.2](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.8.0-alpha.1...bevy_mod_scripting_lua-v0.8.0-alpha.2) - 2024-12-03

### Fixed

- bug when compiling without `teal` feature ([#148](https://github.com/makspll/bevy_mod_scripting/pull/148))

### Other

- Luau support attempt ([#154](https://github.com/makspll/bevy_mod_scripting/pull/154))
- Fix failing doctest ([#146](https://github.com/makspll/bevy_mod_scripting/pull/146))

## [0.8.0-alpha.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.8.0-alpha.0...bevy_mod_scripting_lua-v0.8.0-alpha.1) - 2024-11-10

### Other

- update Cargo.toml dependencies

## [0.7.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.7.0...bevy_mod_scripting_lua-v0.7.1) - 2024-11-03

### Other

- Documentation generation hotfixes ([#130](https://github.com/makspll/bevy_mod_scripting/pull/130))

## [0.7.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.6.0...bevy_mod_scripting_lua-v0.7.0) - 2024-11-03

### Other

- Migrate to bevy 0.14 ([#127](https://github.com/makspll/bevy_mod_scripting/pull/127))
- update metadata
