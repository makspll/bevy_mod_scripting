# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.15.1...bevy_mod_scripting_functions-v0.16.0) - 2025-09-22

### Added

- improved dynamic printing, adds `ReflectDisplayWithTypeInfo` for overriding opaque type printing ([#478](https://github.com/makspll/bevy_mod_scripting/pull/478))
- improve codegen crate calculation, separate bindings into crates ([#467](https://github.com/makspll/bevy_mod_scripting/pull/467))
- pre-expand generated codegen macros ([#462](https://github.com/makspll/bevy_mod_scripting/pull/462))

### Refactored

- extract `bevy_mod_scripting_asset` and `bevy_mod_scripting_display` crates, decouple concerns ([#477](https://github.com/makspll/bevy_mod_scripting/pull/477))
- extract `bevy_mod_scripting_asset` crate, simplify supported extensions logic ([#475](https://github.com/makspll/bevy_mod_scripting/pull/475))
- [**breaking**] refactor dependencies, point at bevy subcrates directly ([#463](https://github.com/makspll/bevy_mod_scripting/pull/463))

## [0.15.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.15.0...bevy_mod_scripting_functions-v0.15.1) - 2025-08-18

### Changed

- Remove asset_path field from ScriptAsset. ([#450](https://github.com/makspll/bevy_mod_scripting/pull/450))

## [0.15.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.14.0...bevy_mod_scripting_functions-v0.15.0) - 2025-08-14

### Added

- [**breaking**] Use the Handles, Luke! ([#427](https://github.com/makspll/bevy_mod_scripting/pull/427)) ([#444](https://github.com/makspll/bevy_mod_scripting/pull/444))

## [0.12.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.11.1...bevy_mod_scripting_functions-v0.12.0) - 2025-04-07

### Added

- [**breaking**] Add `BMSPlugin` group, feature flag for bindings per bevy crate & add script global filter options ([#408](https://github.com/makspll/bevy_mod_scripting/pull/408))

## [0.11.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.10.0...bevy_mod_scripting_functions-v0.11.0) - 2025-03-29

### Added

- optimize `get` and `set` functions, add `MagicFunctions` sub-registry ([#397](https://github.com/makspll/bevy_mod_scripting/pull/397))
- :sparkles: Dynamic Script Components, `register_new_component` binding, `remove_component` no longer requires `ReflectComponent` data ([#379](https://github.com/makspll/bevy_mod_scripting/pull/379))
- overhaul mdbook preprocessor, prettify generated docs, support dummy globals ([#377](https://github.com/makspll/bevy_mod_scripting/pull/377))

## [0.10.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.11...bevy_mod_scripting_functions-v0.10.0) - 2025-03-16

### Added

- add global `types` cache making `get_type_by_name` redundant ([#370](https://github.com/makspll/bevy_mod_scripting/pull/370))
- :sparkles: Parallelizable Script Systems with `Res` and `Query` parameters & Schedule debugging utilities ([#361](https://github.com/makspll/bevy_mod_scripting/pull/361))
