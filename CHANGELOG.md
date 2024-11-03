# Changelog

## [0.6.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting-v0.6.0...bevy_mod_scripting-v0.6.1) - 2024-11-03

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