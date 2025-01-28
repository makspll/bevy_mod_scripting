# Changelog

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
