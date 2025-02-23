# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/makspll/bevy_mod_scripting/compare/v0.2.0-ladfile_builder...v0.2.1-ladfile_builder) - 2025-02-23

### Added

- create `ScriptingDocgenPlugin` to allow exporting `LAD` files + export BMS bindings (#303)
- add `construct` global for constructing arbitrary types & `Union` type (#302)
- add global functions to mdbook, allow documenting arguments and return values (#296)

### Fixed

- remove `reflect_functions` and `file_watcher` flags from bevy dependency (#316)
