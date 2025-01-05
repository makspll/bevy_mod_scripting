# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0-alpha.2](https://github.com/makspll/bevy_mod_scripting/compare/bevy_mod_scripting_functions-v0.9.0-alpha.1...bevy_mod_scripting_functions-v0.9.0-alpha.2) - 2025-01-05

### Fixed

- fix readme
- fix lifetime issue, allow refs

### Other

- remove trailing whitespace
- fmt
- clippy fixes
- clippy fixes
- change query signature slightly
- improve logs and things
- finally
- keep working on docs and fixing bugs
- allow optionally disabling bindings
- pin smol str, begin rhai work
- it doens't exist
- enable bevy input in functions for smol_str
- enable more flags in bevy functions
- make iteration work
- just lookup length for iteration for now
- allow passing more arguments than needed
- implement iterators, and add function passing
- make overloading work for subtraction
- *(codegen)* update bevy bindings (#181)
- add script function registry and update registrations
- implement the rest of reflect reference functions
- remove need for world jerry-rig, use static reference
- get static calls working
- get bindings compiling, add more impls
- *(codegen)* update bevy bindings (#180)
- *(codegen)* update bevy bindings (#179)
- *(codegen)* update bevy bindings (#178)
- select new pre-release version
- clean up versions from last pre-release
- *(codegen)* update bevy bindings (#177)
- imports
- *(codegen)* update bevy bindings (#176)
- *(codegen)* update bevy bindings (#175)
- change imports
- move bindings under different module
- *(codegen)* update bevy bindings (#174)
- *(codegen)* update bevy bindings (#173)
- *(codegen)* update bevy bindings (#172)
- *(codegen)* update bevy bindings (#171)
- *(codegen)* update bevy bindings (#170)
- properly register the top level types too
- work out function type dependency registration basics
- WIP
- re-enable list applies
- handle errors properly
- initial from_script_ref
- shift to bakery paradigm
- remove older stuff
- change core functions to script functions
- refactor accesses slightly
- Big things coming
- WIP
- move script value things into another module
- refactor a bit
- add support for list
- refactor errors considerably
- dynamic get and set calls
- Implement ScriptValue things
- DYNAMIC CALLS WOOOOOOO
