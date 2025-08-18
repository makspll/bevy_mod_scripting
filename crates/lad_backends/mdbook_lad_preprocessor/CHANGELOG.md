# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.1.10-mdbook_lad_preprocessor...v0.2.0-mdbook_lad_preprocessor) - 2025-08-14

### Added

- [**breaking**] Use the Handles, Luke! ([#427](https://github.com/makspll/bevy_mod_scripting/pull/427)) ([#444](https://github.com/makspll/bevy_mod_scripting/pull/444))

## [0.1.7](https://github.com/makspll/bevy_mod_scripting/compare/v0.1.6-mdbook_lad_preprocessor...v0.1.7-mdbook_lad_preprocessor) - 2025-04-07

### Added

- improve errors when entity is unavailable ([#410](https://github.com/makspll/bevy_mod_scripting/pull/410))

### Fixed

- links in type functions ([#412](https://github.com/makspll/bevy_mod_scripting/pull/412))

## [0.1.6](https://github.com/makspll/bevy_mod_scripting/compare/v0.1.5-mdbook_lad_preprocessor...v0.1.6-mdbook_lad_preprocessor) - 2025-03-29

### Fixed

- types links being broken again

## [0.1.5](https://github.com/makspll/bevy_mod_scripting/compare/v0.1.4-mdbook_lad_preprocessor...v0.1.5-mdbook_lad_preprocessor) - 2025-03-29

### Added

- overhaul mdbook preprocessor, prettify generated docs, support dummy globals ([#377](https://github.com/makspll/bevy_mod_scripting/pull/377))

### Fixed

- make all links in the mdbook preprocessor relative ([#392](https://github.com/makspll/bevy_mod_scripting/pull/392))
- mdbook preprocessor links not taking into account root url ([#391](https://github.com/makspll/bevy_mod_scripting/pull/391))

## [0.1.4](https://github.com/makspll/bevy_mod_scripting/compare/v0.1.3-mdbook_lad_preprocessor...v0.1.4-mdbook_lad_preprocessor) - 2025-03-16

### Added

- *(ladfile)* improve globals in LAD format ([#372](https://github.com/makspll/bevy_mod_scripting/pull/372))

## [0.1.3](https://github.com/makspll/bevy_mod_scripting/compare/mdbook_lad_preprocessor-v0.1.2...mdbook_lad_preprocessor-v0.1.3) - 2025-02-28

### Added

- *(lad)* export global functions in `lad` exported plugin & add collapsible sections to pre-processor (#334)

## [0.1.2](https://github.com/makspll/bevy_mod_scripting/compare/mdbook_lad_preprocessor-v0.1.1...mdbook_lad_preprocessor-v0.1.2) - 2025-02-25

### Added

- *(mdbook)* improve mdbook generics behaviour and fix broken links (#319)

## [0.1.1](https://github.com/makspll/bevy_mod_scripting/compare/mdbook_lad_preprocessor-v0.1.0...mdbook_lad_preprocessor-v0.1.1) - 2025-02-23

### Added

- add global functions to mdbook, allow documenting arguments and return values (#296)
- separate `ladfile` into `ladfile_builder` and `ladfile` crates (#293)

## [0.1.0](https://github.com/makspll/bevy_mod_scripting/releases/tag/mdbook_lad_preprocessor-v0.1.0) - 2025-02-15

### Added

- Improve mdbook preprocessor formatting (#290)
- create mdbook backend for LAD files (#287)
