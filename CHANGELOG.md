# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.15.1](https://github.com/makspll/bevy_mod_scripting/compare/v0.15.0...v0.15.1) - 2025-08-18

### Changed

- Remove asset_path field from ScriptAsset. ([#450](https://github.com/makspll/bevy_mod_scripting/pull/450))

### Fixed

- "luau" extension regression ([#453](https://github.com/makspll/bevy_mod_scripting/pull/453))

## [0.15.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.14.0...v0.15.0) - 2025-08-14

### Added

- [**breaking**] Use the Handles, Luke! ([#427](https://github.com/makspll/bevy_mod_scripting/pull/427)) ([#444](https://github.com/makspll/bevy_mod_scripting/pull/444))
# Changelog

## [0.13.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.12.0...v0.13.0) - 2025-07-05

### Added

- Include Entity in `ScriptCallbackResponseEvent` ([#425](https://github.com/makspll/bevy_mod_scripting/pull/425))
- Add on_script_reloaded callback. ([#421](https://github.com/makspll/bevy_mod_scripting/pull/421))
- Warn on unknown language. ([#418](https://github.com/makspll/bevy_mod_scripting/pull/418))

### Other

- Update 0.12.0.md
- Update 0.12.0.md
- Update CHANGELOG.md
- Update 0.12.0.md
- Create 0.12.0.md

## [0.12.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.11.1...v0.12.0) - 2025-04-07

### Added

- improve errors when entity is unavailable ([#410](https://github.com/makspll/bevy_mod_scripting/pull/410))
- [**breaking**] Add `BMSPlugin` group, feature flag for bindings per bevy crate & add script global filter options ([#408](https://github.com/makspll/bevy_mod_scripting/pull/408))
    - The CoreScriptGlobalsPlugin now also stores options for filtering registered globals, which can be changed.
- add option to emit response event on each callback  & `RunScriptCallback` command for "once-off" callbacks ([#403](https://github.com/makspll/bevy_mod_scripting/pull/403))
### Fixed

- `lua54` feature being forced ([#413](https://github.com/makspll/bevy_mod_scripting/pull/413))
- `GetTypeDependency` derive macro using the wrong path for `bms_core` ([#409](https://github.com/makspll/bevy_mod_scripting/pull/409))

### Other

- add script loading benchmark ([#411](https://github.com/makspll/bevy_mod_scripting/pull/411))
- refactor `ReflectReference` internally ([#406](https://github.com/makspll/bevy_mod_scripting/pull/406))
- reduces size of `ScriptValue` to 64 bytes, moves some dynamic function methods into function info ([#404](https://github.com/makspll/bevy_mod_scripting/pull/404))

## [0.11.1](https://github.com/makspll/bevy_mod_scripting/compare/v0.11.0...v0.11.1) - 2025-03-29

### Added

- bump bevy to 0.15.3 ([#401](https://github.com/makspll/bevy_mod_scripting/pull/401))

## [0.11.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.10.0...v0.11.0) - 2025-03-29

### Added

- allow the conversion of lua functions into `ScriptValue` via `DynamicScriptFunction` ([#396](https://github.com/makspll/bevy_mod_scripting/pull/396))
- improve tracing spans, add more benchmarks ([#394](https://github.com/makspll/bevy_mod_scripting/pull/394))
- add `profile_with_tracy` feature which plays nicely with bevy's `bevy/trace_tracy` feature ([#393](https://github.com/makspll/bevy_mod_scripting/pull/393))
- Add initial benchmarks, integrate them into CI & add getters/settters for `Scripts` resource ([#381](https://github.com/makspll/bevy_mod_scripting/pull/381))
- :sparkles: Dynamic Script Components, `register_new_component` binding, `remove_component` no longer requires `ReflectComponent` data ([#379](https://github.com/makspll/bevy_mod_scripting/pull/379))
- [**breaking**] optimize `get` and `set` functions, add `MagicFunctions` sub-registry ([#397](https://github.com/makspll/bevy_mod_scripting/pull/397))
- optimize access map ([#395](https://github.com/makspll/bevy_mod_scripting/pull/395))
- add ScriptValue override for printing opaque values ([#380](https://github.com/makspll/bevy_mod_scripting/pull/380))
- overhaul mdbook preprocessor, prettify generated docs, support dummy globals ([#377](https://github.com/makspll/bevy_mod_scripting/pull/377))

### Fixed

- fix global type cache not containing generic types ([#388](https://github.com/makspll/bevy_mod_scripting/pull/388))

### Other

- switch to hashbrown hashmap in the function registry ([#399](https://github.com/makspll/bevy_mod_scripting/pull/399))
- try play with hashing for access maps ([#398](https://github.com/makspll/bevy_mod_scripting/pull/398))
- allow check creation for bencher

## [0.10.0](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.11...v0.10.0) - 2025-03-16

### Added

- add global `types` cache making `get_type_by_name` redundant ([#370](https://github.com/makspll/bevy_mod_scripting/pull/370))
- :sparkles: Parallelizable Script Systems with `Res` and `Query` parameters & Schedule debugging utilities ([#361](https://github.com/makspll/bevy_mod_scripting/pull/361))
- Add missing `luau` extension, improve extension configuration options ([#366](https://github.com/makspll/bevy_mod_scripting/pull/366))
- *(ladfile)* improve globals in LAD format ([#372](https://github.com/makspll/bevy_mod_scripting/pull/372))
- [**breaking**] re-design `GetTypeDependencies` trait & add `GetTypeDependencies` derive macro ([#369](https://github.com/makspll/bevy_mod_scripting/pull/369))
- shorten import paths ([#367](https://github.com/makspll/bevy_mod_scripting/pull/367))
- allow lua scripts to insert `ScriptComponent`'s ([#359](https://github.com/makspll/bevy_mod_scripting/pull/359))

### Fixed

- [**breaking**] script contexts being completely overwritten on a re-load ([#345](https://github.com/makspll/bevy_mod_scripting/pull/345))
- unit enum variants other than `Option::None` being converted into `ScriptValue::Unit` ([#360](https://github.com/makspll/bevy_mod_scripting/pull/360))

### Other

- [**breaking**] Merge `ScriptContexts<T>` into `Scripts<T>` + Remove `Sync` bound from Contexts ([#350](https://github.com/makspll/bevy_mod_scripting/pull/350))

## [0.9.11](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.10...v0.9.11) - 2025-03-03

### Fixed

- plugin registration order affecting which globals are exported ([#346](https://github.com/makspll/bevy_mod_scripting/pull/346))

## [0.9.10](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.9...v0.9.10) - 2025-03-03

### Added

- add `map_get` function for cloning and returning values on a map ([#343](https://github.com/makspll/bevy_mod_scripting/pull/343))
- *(bms,ladfile_builder)* introduce app global instance registry and export them in `ladfile_builder` ([#340](https://github.com/makspll/bevy_mod_scripting/pull/340))

## [0.9.9](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.8...v0.9.9) - 2025-02-28

### Added

- Add `GlobalNamespace::system_builder`, `World::add_system` and allow dynamic system creation (#335)
- add `WithWorldGuard` and `HandlerContext` system parameters (#327)
- add test for construct using unit struct (#328)
- support setting hashmaps via reflection (#330)
- allow hashmap `FromScript` from list of tuples (#332)

## [0.9.8](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.7...v0.9.8) - 2025-02-25

### Added

- Allow trailing comma in callback_labels. (#325)

### Fixed

- `enable_context_sharing` not returning the plugin like a real builder

## [0.9.7](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.6...v0.9.7) - 2025-02-23

### Added

- create `ScriptingDocgenPlugin` to allow exporting `LAD` files + export BMS bindings (#303)
- add global functions to mdbook, allow documenting arguments and return values (#296)
- separate `ladfile` into `ladfile_builder` and `ladfile` crates (#293)
- add `construct` global for constructing arbitrary types & `Union` type (#302)
- pre-register reflected components with the world at finalize (#314)
- add allocator diagnostics (#305)
- improve warning on missing asset (#295)

### Fixed

- functions not releasing accesses correctly on error (#315)
- remove `reflect_functions` and `file_watcher` flags from bevy dependency (#316)

## [0.9.6](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.5...v0.9.6) - 2025-02-15

### Added

- create mdbook backend for LAD files (#287)

### Fixed

- compilation error with `bevy/trace_tracy` (#289)

## [0.9.5](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.4...v0.9.5) - 2025-02-12

### Added

- update bevy to 0.15.2 (#280)

## [0.9.4](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.3...v0.9.4) - 2025-02-12

### Added

- create `Language Agnostic Declaration` file format and `ladfile` crate (#274)
- Add `script_bindings` impl block derive macro (#263)
- add `TypedThrough` abstraction to function meta, and refactor (#272)
- refactor generated bindings to use new derive macro (#268)
- refactor core bindings to use new derive macro (#267)

### Fixed

- fix tracy compile errors and add tracy buid to CI (#277)
- don't use `new_unregistered` for most of core bindings macros (#270)

## [0.9.3](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.2...v0.9.3) - 2025-02-08

### Added

- add static scripts which do not need to be attached to entities to be run (#253)
- add recipient for specific language (#250)

## [0.9.2](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.1...v0.9.2) - 2025-02-08

### Added

- make `extractors` module non-public (#251)

### Fixed

- add missing extensions in the asset loader (#254)

## [0.9.1](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.0...v0.9.1) - 2025-02-01

### Fixed

- bump `bevy` to 0.15.1 (#241)

## [0.9.0-alpha.9](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.0-alpha.8...v0.9.0-alpha.9) - 2025-01-28

### Fixed

- prevent allocation and component ID ranges from overlapping (#230)

## [0.9.0-alpha.8](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.0-alpha.7...v0.9.0-alpha.8) - 2025-01-27

### Added

- re-implement rhai again (#222)
- add `ScriptValue::Map` and create appropriate conversions in lua and rhai (#229)
- Add `functions` script method, and create function info scaffolding (#228)
- Call custom `get` and `set` functions on the type when indexing. (#226)
- Add `optional` arguments to script functions (#225)
- Add world.with_or_insert_component_mut() (#223)

## [0.9.0-alpha.7](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.0-alpha.6...v0.9.0-alpha.7) - 2025-01-20

### Added

- [**breaking**] Remove `WorldCallbackAccess` & Combine context args for dynamic functions into one `FunctionCallContext` (#219)
- Add component `upsert` function (#218)

## [0.9.0-alpha.6](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.0-alpha.5...v0.9.0-alpha.6) - 2025-01-19

### Added

- Don't panic! (#216)

## [0.9.0-alpha.5](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.0-alpha.4...v0.9.0-alpha.5) - 2025-01-19

### Fixed

- Fix missing functions in codegen (#210)

## [0.9.0-alpha.3](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.0-alpha.2...v0.9.0-alpha.3) - 2025-01-14

### Added

- Improvements to BMS in multi-language context (#194)
- Implement global namespace registration (#202)
- make script contexts public (#193)

## [0.9.0-alpha.2](https://github.com/makspll/bevy_mod_scripting/compare/v0.9.0-alpha.1...v0.9.0-alpha.2) - 2025-01-05

### Added

- Dynamic function registry and dynamic function calls
- `bevy_mod_scripting_functions` crate added, containing built-in dynamic functions callable from scripts
- Lua dynamic function call mechanism
- Dynamic functions automatically register their argument and return types with the type registry
- Added set of `IntoScript`, `FromScript`, `IntoScriptRef`, and `FromScriptRef` traits 
- Added `ScriptAllocator` to manage lifetimes of non-world stored types (such as `Vec2` created via scripts etc..)
- Added `AccessMap` dynamic safety mechanism, every access is now small, and does not require mutexing the entire world

### Changed
- Complete plugin re-write, expect breakages everywhere
- `prelude` imports removed
- `ScriptValue` abstraction replacing the concept of a generic event argument type. Each event payload is a `ScriptValue`
- `world` is now a static reference, `world:function` calls must be replaced with `world.function` calls
- Documentation generation was temporarilly removed
- `Teal` and `Tealr` was removed
- `bevy_mod_scripting_derive`, `bevy_mod_scripting_common` and other derive crates as well as `bevy_event_priority` and `bevy_script_api` crates were removed
- Temporarilly suspended full rhai and rune support until next non-alpha release
- Removed Deferred reflection mechanism
- Added `mdbook` documentation book
- Removed `APIProvider` traits in favour of various configuration resources
- Specific registration of `Vec<T>` and `Option<T>` via `register_lua_vec` etc.. is no longer necessary, reflection *just* works on all registered types
- Expanded core library of `ReflectReference` functions
- Removed `LuaProxyable` abstraction and all custom type data, everything is now driven via normal reflection
- All references are now represented via either references to the world or to a `ScriptAllocator`
- Accessing anything in the world requires claiming the appropriate `AccessMap` locks to do so safely (which is abstracted away with various utility functions)
- And much more

## [0.8.0-alpha.2](https://github.com/makspll/bevy_mod_scripting/compare/v0.8.0-alpha.1...v0.8.0-alpha.2) - 2024-12-03

### Fixed

- bug when compiling without `teal` feature ([#148](https://github.com/makspll/bevy_mod_scripting/pull/148))

### Other

- Small fixes ([#155](https://github.com/makspll/bevy_mod_scripting/pull/155))
- Luau support attempt ([#154](https://github.com/makspll/bevy_mod_scripting/pull/154))
- Bump bevy & bevy console ([#153](https://github.com/makspll/bevy_mod_scripting/pull/153))
- Fix failing doctest ([#146](https://github.com/makspll/bevy_mod_scripting/pull/146))
- update Cargo.toml dependencies

## [0.8.0-alpha.1](https://github.com/makspll/bevy_mod_scripting/compare/v0.8.0-alpha.0...v0.8.0-alpha.1) - 2024-11-10

### Other

- Bump Bevy release candidate ([#143](https://github.com/makspll/bevy_mod_scripting/pull/143))
- update Cargo.toml dependencies

## [0.7.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting-v0.7.0...bevy_mod_scripting-v0.7.1) - 2024-11-03

### Other

- Documentation generation hotfixes ([#130](https://github.com/makspll/bevy_mod_scripting/pull/130))

## [0.7.0](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting-v0.6.0...bevy_mod_scripting-v0.7.0) - 2024-11-03

### Other

- Add dynamic query examples ([#120](https://github.com/makspll/bevy_mod_scripting/pull/120))
- Migrate to bevy 0.14 ([#127](https://github.com/makspll/bevy_mod_scripting/pull/127))
- Fix Broken Example ([#123](https://github.com/makspll/bevy_mod_scripting/pull/123))
- Fix cross-platform CI.yml ([#111](https://github.com/makspll/bevy_mod_scripting/pull/111))
- update metadata

## v0.2.2
- Bump `tealr_doc_gen` and `tealr` versions
- Change bevy dependency semver to "0.9"
## v0.2.1
### Added
- Automatic documentation publishing for lua Bevy api 
- Added binary for generating documentation
### Fixed
- Fixed bug where errors in documenation generation didn't propagate properly
- Fixed broken link in readme.md

## v0.2.0
### Added
- Added support for the Bevy API for Rhai
- Foundations laid for proxy macro for Rhai
- Added `game_of_life` and `bevy_api` examples for Rhai
- Added more hooks for APIProviders. `entity` and `world` constants are now set by API providers and hence you must register the `BevyAPIProvider` for your scripting language to access those. This let's us accomodate people who want barebones scripting without access to Bevy, or roll their own fully fledged API's.
### Changed
- Revived `console_integration` examples
- Major changes to low level API's
- Major import structure changes
- Split crate into smaller crates
- Added more control over what's pulled into the dependency tree with finely grained features

## v0.1.1
### Added 
- Added `CHANGELOG.md`
- Incorporated `cargo release`
### Changed
- Fixed broken example links in `readme.md`

## v0.1.0
Initial version
