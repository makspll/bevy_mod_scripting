# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
