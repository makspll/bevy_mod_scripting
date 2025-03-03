# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.10](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_derive-v0.9.9...bevy_mod_scripting_derive-v0.9.10) - 2025-03-03

### Added

- *(bms,ladfile_builder)* introduce app global instance registry and export them in `ladfile_builder` ([#340](https://github.com/makspll/bevy_mod_scripting/pull/340))

## [0.9.8](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_derive-v0.9.7...bevy_mod_scripting_derive-v0.9.8) - 2025-02-25

### Fixed

- Generate IntoScript implementation with the correct path (#326)

## [0.9.7](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_derive-v0.9.6...bevy_mod_scripting_derive-v0.9.7) - 2025-02-23

### Added

- Add `TypedThrough` and `IntoScript` derive macros (#294)

## [0.9.4](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_derive-v0.9.3...bevy_mod_scripting_derive-v0.9.4) - 2025-02-12

### Added

- refactor generated bindings to use new derive macro (#268)
- refactor core bindings to use new derive macro (#267)
