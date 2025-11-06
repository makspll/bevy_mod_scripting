# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.1](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_display-v0.16.0...bevy_mod_scripting_display-v0.16.1) - 2025-11-06

### Added

- Replace `ParsedPath` with custom `ReferencePath`, support `Map` and `Set` references with arbitrary types ([#491](https://github.com/makspll/bevy_mod_scripting/pull/491))
- registered callbacks via `register_callback`, and `bevy_mod_scripting_script` crate. ([#490](https://github.com/makspll/bevy_mod_scripting/pull/490))
- Asset references, `world.get_asset` and `world.has_asset` bindings ([#484](https://github.com/makspll/bevy_mod_scripting/pull/484))
