# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0-alpha.2](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.0-alpha.1...bevy_mod_scripting_core-v0.9.0-alpha.2) - 2025-01-05

### Fixed

- fix test and small refactor

### Other

- improve xtasks and start integrating with ci
- make display without world more sensible
- further clean up
- remove preludes, remove more crates
- remove unused dependencies
- fmt
- more clippy
- clippy fixes
- clippy fixes
- cleanup some imports
- change query signature slightly
- add on load and unload hooks
- improve logs and things
- finally
- keep working on docs and fixing bugs
- make the generics more tenable, and update docs
- start cleaning up examples, use ScriptValue as argument for all things
- add more conversions
- change handle to weak so unloading strong handle is enough to delete script, and update docs
- add error conversions for rhai

## [0.8.0-alpha.2](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.8.0-alpha.1...bevy_mod_scripting_core-v0.8.0-alpha.2) - 2024-12-03

### Other

- Bump bevy & bevy console ([#153](https://github.com/makspll/bevy_mod_scripting/pull/153))

## [0.8.0-alpha.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.8.0-alpha.0...bevy_mod_scripting_core-v0.8.0-alpha.1) - 2024-11-10

### Other

- update Cargo.toml dependencies

## [0.7.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.6.0...bevy_mod_scripting_core-v0.7.0) - 2024-11-03

### Other

- Send ScriptErrorEvent when load fails. ([#125](https://github.com/makspll/bevy_mod_scripting/pull/125))
- Migrate to bevy 0.14 ([#127](https://github.com/makspll/bevy_mod_scripting/pull/127))
- update metadata
