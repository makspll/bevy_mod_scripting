# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.9.11...bevy_mod_scripting_core-v0.10.0) - 2025-03-16

### Added

- *(ladfile)* improve globals in LAD format ([#372](https://github.com/makspll/bevy_mod_scripting/pull/372))
- add global `types` cache making `get_type_by_name` redundant ([#370](https://github.com/makspll/bevy_mod_scripting/pull/370))
- [**breaking**] re-design `GetTypeDependencies` trait & add `GetTypeDependencies` derive macro ([#369](https://github.com/makspll/bevy_mod_scripting/pull/369))
- shorten import paths ([#367](https://github.com/makspll/bevy_mod_scripting/pull/367))
- Add missing `luau` extension, improve extension configuration options ([#366](https://github.com/makspll/bevy_mod_scripting/pull/366))
- allow lua scripts to insert `ScriptComponent`'s ([#359](https://github.com/makspll/bevy_mod_scripting/pull/359))
- :sparkles: Parallelizable Script Systems with `Res` and `Query` parameters & Schedule debugging utilities ([#361](https://github.com/makspll/bevy_mod_scripting/pull/361))

### Fixed

- supported extensions not including default extensions [SKIP_CHANGELOG] ([#373](https://github.com/makspll/bevy_mod_scripting/pull/373))
- unit enum variants other than `Option::None` being converted into `ScriptValue::Unit` ([#360](https://github.com/makspll/bevy_mod_scripting/pull/360))

### Other

- [**breaking**] Merge `ScriptContexts<T>` into `Scripts<T>` + Remove `Sync` bound from Contexts ([#350](https://github.com/makspll/bevy_mod_scripting/pull/350))
