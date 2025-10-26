# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_derive-v0.15.1...bevy_mod_scripting_derive-v0.16.0) - 2025-09-22

### Added

- improved dynamic printing, adds `ReflectDisplayWithTypeInfo` for overriding opaque type printing ([#478](https://github.com/makspll/bevy_mod_scripting/pull/478))

### Refactored

- extract `bevy_mod_scripting_asset` and `bevy_mod_scripting_display` crates, decouple concerns ([#477](https://github.com/makspll/bevy_mod_scripting/pull/477))
- [**breaking**] refactor dependencies, point at bevy subcrates directly ([#463](https://github.com/makspll/bevy_mod_scripting/pull/463))

## [0.12.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_derive-v0.11.1...bevy_mod_scripting_derive-v0.12.0) - 2025-04-07

### Fixed

- Derive `GetTypeDependency` without explicit "core" dependency. ([#409](https://github.com/makspll/bevy_mod_scripting/pull/409))

## [0.11.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_derive-v0.10.0...bevy_mod_scripting_derive-v0.11.0) - 2025-03-29

### Added

- overhaul mdbook preprocessor, prettify generated docs, support dummy globals ([#377](https://github.com/makspll/bevy_mod_scripting/pull/377))

## [0.10.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_derive-v0.9.11...bevy_mod_scripting_derive-v0.10.0) - 2025-03-16

### Added

- [**breaking**] re-design `GetTypeDependencies` trait & add `GetTypeDependencies` derive macro ([#369](https://github.com/makspll/bevy_mod_scripting/pull/369))
