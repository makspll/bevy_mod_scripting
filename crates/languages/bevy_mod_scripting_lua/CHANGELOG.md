# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.15.1...bevy_mod_scripting_lua-v0.16.0) - 2025-09-22

### Added

- improve and formalize script processing pipeline ([#481](https://github.com/makspll/bevy_mod_scripting/pull/481))
- improved dynamic printing, adds `ReflectDisplayWithTypeInfo` for overriding opaque type printing ([#478](https://github.com/makspll/bevy_mod_scripting/pull/478))

### Refactored

- extract `bevy_mod_scripting_asset` and `bevy_mod_scripting_display` crates, decouple concerns ([#477](https://github.com/makspll/bevy_mod_scripting/pull/477))
- extract `bevy_mod_scripting_asset` crate, simplify supported extensions logic ([#475](https://github.com/makspll/bevy_mod_scripting/pull/475))
- remove `HandlerCtxt`, wrap `ScriptContext<P>` in Arc ([#474](https://github.com/makspll/bevy_mod_scripting/pull/474))
- remove `StaticScripts`  resource ([#473](https://github.com/makspll/bevy_mod_scripting/pull/473))
- modify `ContextLoadFn` & `ContextReloadFn` & `HandlerFn` to use `WorldId` instead of direct config ([#472](https://github.com/makspll/bevy_mod_scripting/pull/472))
- remove `RuntimeContainer` & `RuntimeSettings`, add plugin runtime to static world local settings ([#471](https://github.com/makspll/bevy_mod_scripting/pull/471))
- add world-local static plugin config, remove `ContextLoadingSettings` resource ([#470](https://github.com/makspll/bevy_mod_scripting/pull/470))
- [**breaking**] refactor dependencies, point at bevy subcrates directly ([#463](https://github.com/makspll/bevy_mod_scripting/pull/463))
- inline `CallbackBuilder<P>` into `IntoScriptPluginParams` at compile time ([#456](https://github.com/makspll/bevy_mod_scripting/pull/456))
- inline `CallbackSettings<P>` into `IntoScriptPluginParam` at compile time ([#455](https://github.com/makspll/bevy_mod_scripting/pull/455))

## [0.15.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.14.0...bevy_mod_scripting_lua-v0.15.0) - 2025-08-14

### Added

- [**breaking**] Use the Handles, Luke! ([#427](https://github.com/makspll/bevy_mod_scripting/pull/427)) ([#444](https://github.com/makspll/bevy_mod_scripting/pull/444))

## [0.11.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.10.0...bevy_mod_scripting_lua-v0.11.0) - 2025-03-29

### Added

- optimize `get` and `set` functions, add `MagicFunctions` sub-registry ([#397](https://github.com/makspll/bevy_mod_scripting/pull/397))
- allow the conversion of lua functions into `ScriptValue` via `DynamicScriptFunction` ([#396](https://github.com/makspll/bevy_mod_scripting/pull/396))
- Add initial benchmarks, integrate them into CI & add getters/settters for `Scripts` resource ([#381](https://github.com/makspll/bevy_mod_scripting/pull/381))
- :sparkles: Dynamic Script Components, `register_new_component` binding, `remove_component` no longer requires `ReflectComponent` data ([#379](https://github.com/makspll/bevy_mod_scripting/pull/379))

## [0.10.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_lua-v0.9.11...bevy_mod_scripting_lua-v0.10.0) - 2025-03-16

### Added

- Add missing `luau` extension, improve extension configuration options ([#366](https://github.com/makspll/bevy_mod_scripting/pull/366))

### Fixed

- [**breaking**] script contexts being completely overwritten on a re-load ([#345](https://github.com/makspll/bevy_mod_scripting/pull/345))

### Other

- [**breaking**] Merge `ScriptContexts<T>` into `Scripts<T>` + Remove `Sync` bound from Contexts ([#350](https://github.com/makspll/bevy_mod_scripting/pull/350))
