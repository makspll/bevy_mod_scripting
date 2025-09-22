# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.5.1-ladfile_builder...v0.6.0-ladfile_builder) - 2025-09-22

### Added

- improved dynamic printing, adds `ReflectDisplayWithTypeInfo` for overriding opaque type printing ([#478](https://github.com/makspll/bevy_mod_scripting/pull/478))

### Refactored

- extract `bevy_mod_scripting_asset` and `bevy_mod_scripting_display` crates, decouple concerns ([#477](https://github.com/makspll/bevy_mod_scripting/pull/477))
- [**breaking**] refactor dependencies, point at bevy subcrates directly ([#463](https://github.com/makspll/bevy_mod_scripting/pull/463))

## [0.5.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.4.0-ladfile_builder...v0.5.0-ladfile_builder) - 2025-08-14

### Added

- [**breaking**] Use the Handles, Luke! ([#427](https://github.com/makspll/bevy_mod_scripting/pull/427)) ([#444](https://github.com/makspll/bevy_mod_scripting/pull/444))

## [0.4.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.3.4-ladfile_builder...v0.4.0-ladfile_builder) - 2025-08-13

### Added

- [**breaking**] Update to Bevy 0.16 ([#422](https://github.com/makspll/bevy_mod_scripting/pull/422))

## [0.3.1](https://github.com/makspll/bevy_mod_scripting/compare/v0.3.0-ladfile_builder...v0.3.1-ladfile_builder) - 2025-03-29

### Added

- bump bevy to 0.15.3 ([#401](https://github.com/makspll/bevy_mod_scripting/pull/401))

## [0.3.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.2.6-ladfile_builder...v0.3.0-ladfile_builder) - 2025-03-29

### Added

- overhaul mdbook preprocessor, prettify generated docs, support dummy globals ([#377](https://github.com/makspll/bevy_mod_scripting/pull/377))

## [0.2.6](https://github.com/makspll/bevy_mod_scripting/compare/v0.2.5-ladfile_builder...v0.2.6-ladfile_builder) - 2025-03-16

### Added

- *(ladfile)* improve globals in LAD format ([#372](https://github.com/makspll/bevy_mod_scripting/pull/372))

## [0.2.4](https://github.com/makspll/bevy_mod_scripting/compare/v0.2.3-ladfile_builder...v0.2.4-ladfile_builder) - 2025-03-03

### Added

- *(bms,ladfile_builder)* introduce app global instance registry and export them in `ladfile_builder` ([#340](https://github.com/makspll/bevy_mod_scripting/pull/340))

## [0.2.3](https://github.com/makspll/bevy_mod_scripting/compare/v0.2.2-ladfile_builder...v0.2.3-ladfile_builder) - 2025-02-28

### Added

- *(lad)* export global functions in `lad` exported plugin & add collapsible sections to pre-processor (#334)

## [0.2.1](https://github.com/makspll/bevy_mod_scripting/compare/v0.2.0-ladfile_builder...v0.2.1-ladfile_builder) - 2025-02-23

### Added

- create `ScriptingDocgenPlugin` to allow exporting `LAD` files + export BMS bindings (#303)
- add `construct` global for constructing arbitrary types & `Union` type (#302)
- add global functions to mdbook, allow documenting arguments and return values (#296)

### Fixed

- remove `reflect_functions` and `file_watcher` flags from bevy dependency (#316)
