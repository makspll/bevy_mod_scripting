# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.24.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.23.0...bevy_mod_scripting_functions-v0.24.0) - 2026-10-18

### Added

- [**breaking**] namespace members and builtins replace script globals ([#635](https://github.com/makspll/bevy_mod_scripting/pull/635))
- [**breaking**] reach types through category namespaces instead of globals ([#629](https://github.com/makspll/bevy_mod_scripting/pull/629))
- [**breaking**] hand-written gizmos, animation, audio and state bindings ([#628](https://github.com/makspll/bevy_mod_scripting/pull/628))
- [**breaking**] hand-written sprite, text and ui bindings ([#627](https://github.com/makspll/bevy_mod_scripting/pull/627))
- [**breaking**] hand-written asset, camera and render bindings ([#626](https://github.com/makspll/bevy_mod_scripting/pull/626))
- [**breaking**] hand-written color, input and window bindings ([#625](https://github.com/makspll/bevy_mod_scripting/pull/625))
- [**breaking**] hand-written time and transform bindings ([#624](https://github.com/makspll/bevy_mod_scripting/pull/624))
- [**breaking**] hand-written ecs bindings, generic eq and clone ([#623](https://github.com/makspll/bevy_mod_scripting/pull/623))
- [**breaking**] hand-written math bindings ([#621](https://github.com/makspll/bevy_mod_scripting/pull/621))
- [**breaking**] remove the deprecated types global and short names in get_type_by_name ([#615](https://github.com/makspll/bevy_mod_scripting/pull/615))

### Other

- [**breaking**] delete allocator + other optimisations ([#637](https://github.com/makspll/bevy_mod_scripting/pull/637))

### Refactored

- [**breaking**] language-agnostic dispatch, type values and shared currying ([#632](https://github.com/makspll/bevy_mod_scripting/pull/632))

## [0.23.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.22.0...bevy_mod_scripting_functions-v0.23.0) - 2026-10-14

### Added

- script observers and script defined events, `<run/queue>_script_callback` added ([#612](https://github.com/makspll/bevy_mod_scripting/pull/612))
- [**breaking**] script namespaces, a shared type catalogue, import_namespace and export_namespace ([#611](https://github.com/makspll/bevy_mod_scripting/pull/611))
- insert_bundle and spawn_bundle for reflected Rust bundles ([#609](https://github.com/makspll/bevy_mod_scripting/pull/609))
- [**breaking**] add `run_script_callback` and `queue_script_callback` bindings, allowing scripts to call each other and return values flexibly ([#575](https://github.com/makspll/bevy_mod_scripting/pull/575))
- shared `new` constructors on every type and enum variant referenced function lookups ([#564](https://github.com/makspll/bevy_mod_scripting/pull/564))
- support triggering `events` and sending `messages` from new script bindings ([#561](https://github.com/makspll/bevy_mod_scripting/pull/561))

### Refactored

- name the type registration union and add instance and reserved function lookups ([#563](https://github.com/makspll/bevy_mod_scripting/pull/563))

## [0.22.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.21.1...bevy_mod_scripting_functions-v0.22.0) - 2026-10-11

### Added

- Bevy 0.20 ([#554](https://github.com/makspll/bevy_mod_scripting/pull/554))

## [0.20.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.19.0...bevy_mod_scripting_functions-v0.20.0) - 2026-06-24

### Added

- [**breaking**] extract `bevy_mod_scripting_world` decoupling world abstractions ([#529](https://github.com/makspll/bevy_mod_scripting/pull/529))
- implement variadics in bindings via `VariadicTuple` and add `ScriptValue::Tuple` ([#527](https://github.com/makspll/bevy_mod_scripting/pull/527))
- logging bindings (`info`, `warn`, `debug`, `trace`, `error`) ([#526](https://github.com/makspll/bevy_mod_scripting/pull/526))
- [**breaking**] Rename `Val`, `Ref`, `Mut`  ([#525](https://github.com/makspll/bevy_mod_scripting/pull/525))

## [0.19.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.18.0...bevy_mod_scripting_functions-v0.19.0) - 2026-01-25

### Added

- [**breaking**] bevy 0.18 ([#507](https://github.com/makspll/bevy_mod_scripting/pull/507))

## [0.18.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.17.0...bevy_mod_scripting_functions-v0.18.0) - 2026-01-25

### Added

- [**breaking**] Bevy 0.17 ([#498](https://github.com/makspll/bevy_mod_scripting/pull/498))

## [0.16.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.16.0...bevy_mod_scripting_functions-v0.16.1) - 2025-11-06

### Added

- registered callbacks via `register_callback`, and `bevy_mod_scripting_script` crate. ([#490](https://github.com/makspll/bevy_mod_scripting/pull/490))
- Asset references, `world.get_asset` and `world.has_asset` bindings ([#484](https://github.com/makspll/bevy_mod_scripting/pull/484))

## [0.16.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.15.1...bevy_mod_scripting_functions-v0.16.0) - 2025-09-22

### Added

- improved dynamic printing, adds `ReflectDisplayWithTypeInfo` for overriding opaque type printing ([#478](https://github.com/makspll/bevy_mod_scripting/pull/478))
- improve codegen crate calculation, separate bindings into crates ([#467](https://github.com/makspll/bevy_mod_scripting/pull/467))
- pre-expand generated codegen macros ([#462](https://github.com/makspll/bevy_mod_scripting/pull/462))

### Refactored

- extract `bevy_mod_scripting_asset` and `bevy_mod_scripting_display` crates, decouple concerns ([#477](https://github.com/makspll/bevy_mod_scripting/pull/477))
- extract `bevy_mod_scripting_asset` crate, simplify supported extensions logic ([#475](https://github.com/makspll/bevy_mod_scripting/pull/475))
- [**breaking**] refactor dependencies, point at bevy subcrates directly ([#463](https://github.com/makspll/bevy_mod_scripting/pull/463))

## [0.15.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.15.0...bevy_mod_scripting_functions-v0.15.1) - 2025-08-18

### Changed

- Remove asset_path field from ScriptAsset. ([#450](https://github.com/makspll/bevy_mod_scripting/pull/450))

## [0.15.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.14.0...bevy_mod_scripting_functions-v0.15.0) - 2025-08-14

### Added

- [**breaking**] Use the Handles, Luke! ([#427](https://github.com/makspll/bevy_mod_scripting/pull/427)) ([#444](https://github.com/makspll/bevy_mod_scripting/pull/444))

## [0.12.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.11.1...bevy_mod_scripting_functions-v0.12.0) - 2025-04-07

### Added

- [**breaking**] Add `BMSPlugin` group, feature flag for bindings per bevy crate & add script global filter options ([#408](https://github.com/makspll/bevy_mod_scripting/pull/408))

## [0.11.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.10.0...bevy_mod_scripting_functions-v0.11.0) - 2025-03-29

### Added

- optimize `get` and `set` functions, add `MagicFunctions` sub-registry ([#397](https://github.com/makspll/bevy_mod_scripting/pull/397))
- :sparkles: Dynamic Script Components, `register_new_component` binding, `remove_component` no longer requires `ReflectComponent` data ([#379](https://github.com/makspll/bevy_mod_scripting/pull/379))
- overhaul mdbook preprocessor, prettify generated docs, support dummy globals ([#377](https://github.com/makspll/bevy_mod_scripting/pull/377))

## [0.10.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.11...bevy_mod_scripting_functions-v0.10.0) - 2025-03-16

### Added

- add global `types` cache making `get_type_by_name` redundant ([#370](https://github.com/makspll/bevy_mod_scripting/pull/370))
- :sparkles: Parallelizable Script Systems with `Res` and `Query` parameters & Schedule debugging utilities ([#361](https://github.com/makspll/bevy_mod_scripting/pull/361))
