# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.5.0-ladfile...v0.6.0-ladfile) - 2025-09-22

### Added

- improved dynamic printing, adds `ReflectDisplayWithTypeInfo` for overriding opaque type printing ([#478](https://github.com/makspll/bevy_mod_scripting/pull/478))

### Refactored

- [**breaking**] refactor dependencies, point at bevy subcrates directly ([#463](https://github.com/makspll/bevy_mod_scripting/pull/463))

## [0.5.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.4.0-ladfile...v0.5.0-ladfile) - 2025-03-29

### Added

- overhaul mdbook preprocessor, prettify generated docs, support dummy globals ([#377](https://github.com/makspll/bevy_mod_scripting/pull/377))

## [0.4.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.3.1-ladfile...v0.4.0-ladfile) - 2025-03-16

### Added

- *(ladfile)* improve globals in LAD format ([#372](https://github.com/makspll/bevy_mod_scripting/pull/372))

## [0.3.1](https://github.com/makspll/bevy_mod_scripting/compare/v0.3.0-ladfile...v0.3.1-ladfile) - 2025-02-25

### Added

- *(mdbook)* improve mdbook generics behaviour and fix broken links (#319)

## [0.3.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.2.0-ladfile...v0.3.0-ladfile) - 2025-02-23

### Added

- add `construct` global for constructing arbitrary types & `Union` type (#302)
- add global functions to mdbook, allow documenting arguments and return values (#296)
- separate `ladfile` into `ladfile_builder` and `ladfile` crates (#293)

## [0.2.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.1.1-ladfile...v0.2.0-ladfile) - 2025-02-15

### Added

- create mdbook backend for LAD files (#287)
- add global instances to `LAD` format (#283)

## [0.1.1](https://github.com/makspll/bevy_mod_scripting/compare/v0.1.0-ladfile...v0.1.1-ladfile) - 2025-02-12

### Added

- update bevy to 0.15.2 (#280)

## [0.1.0](https://github.com/makspll/bevy_mod_scripting/releases/tag/v0.1.0-ladfile) - 2025-02-12

### Added

- create `Language Agnostic Declaration` file format and `ladfile` crate (#274)
