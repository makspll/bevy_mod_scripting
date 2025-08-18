# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.15.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.15.0...bevy_mod_scripting_core-v0.15.1) - 2025-08-18

### Changed

- Remove asset_path field from ScriptAsset. ([#450](https://github.com/makspll/bevy_mod_scripting/pull/450))

### Fixed

- "luau" extension regression ([#453](https://github.com/makspll/bevy_mod_scripting/pull/453))

## [0.15.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.14.0...bevy_mod_scripting_core-v0.15.0) - 2025-08-14

### Added

- [**breaking**] Use the Handles, Luke! ([#427](https://github.com/makspll/bevy_mod_scripting/pull/427)) ([#444](https://github.com/makspll/bevy_mod_scripting/pull/444))

## [0.13.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.12.0...bevy_mod_scripting_core-v0.13.0) - 2025-07-05

### Added

- Include Entity in `ScriptCallbackResponseEvent` ([#425](https://github.com/makspll/bevy_mod_scripting/pull/425))
- Add on_script_reloaded callback. ([#421](https://github.com/makspll/bevy_mod_scripting/pull/421))
- Warn on unknown language. ([#418](https://github.com/makspll/bevy_mod_scripting/pull/418))

## [0.12.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.11.1...bevy_mod_scripting_core-v0.12.0) - 2025-04-07

### Added

- improve errors when entity is unavailable ([#410](https://github.com/makspll/bevy_mod_scripting/pull/410))
- [**breaking**] Add `BMSPlugin` group, feature flag for bindings per bevy crate & add script global filter options ([#408](https://github.com/makspll/bevy_mod_scripting/pull/408))
- add option to emit response event on each callback  & `RunScriptCallback` command for "once-off" callbacks ([#403](https://github.com/makspll/bevy_mod_scripting/pull/403))

### Fixed

- `lua54` feature being forced ([#413](https://github.com/makspll/bevy_mod_scripting/pull/413))

### Other

- refactor `ReflectReference` internally ([#406](https://github.com/makspll/bevy_mod_scripting/pull/406))
- reduces size of `ScriptValue` to 64 bytes, moves some dynamic function methods into function info ([#404](https://github.com/makspll/bevy_mod_scripting/pull/404))

## [0.11.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_core-v0.10.0...bevy_mod_scripting_core-v0.11.0) - 2025-03-29

### Added

- optimize access map ([#395](https://github.com/makspll/bevy_mod_scripting/pull/395))
- optimize `get` and `set` functions, add `MagicFunctions` sub-registry ([#397](https://github.com/makspll/bevy_mod_scripting/pull/397))
- improve tracing spans, add `profile_with_tracy` feature flag ([#394](https://github.com/makspll/bevy_mod_scripting/pull/394))
- add `profile_with_tracy` feature which plays nicely with bevy's `bevy/trace_tracy` feature ([#393](https://github.com/makspll/bevy_mod_scripting/pull/393))
- Add initial benchmarks, integrate them into CI & add getters/settters for `Scripts` resource ([#381](https://github.com/makspll/bevy_mod_scripting/pull/381))
- add ScriptValue override for printing opaque values ([#380](https://github.com/makspll/bevy_mod_scripting/pull/380))
- :sparkles: Dynamic Script Components, `register_new_component` binding, `remove_component` no longer requires `ReflectComponent` data ([#379](https://github.com/makspll/bevy_mod_scripting/pull/379))
- overhaul mdbook preprocessor, prettify generated docs, support dummy globals ([#377](https://github.com/makspll/bevy_mod_scripting/pull/377))

### Fixed

- fix global type cache not containing generic types ([#388](https://github.com/makspll/bevy_mod_scripting/pull/388))

### Other

- switch to hashbrown hashmap in the function registry ([#399](https://github.com/makspll/bevy_mod_scripting/pull/399))
- try play with hashing for access maps ([#398](https://github.com/makspll/bevy_mod_scripting/pull/398))
- allow check creation for bencher

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
