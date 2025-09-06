<img src="./badges/coverage.svg" width="175"> [^2] <img src="./badges/no-panics.svg" width="175"> [^3]
---

<p align="center">
    <img src="./logo_bevy_scripting.svg" width="250" onerror="this.onerror=null; this.src='https://raw.githubusercontent.com/makspll/bevy_mod_scripting/main/logo_bevy_scripting.svg'">
</p>

# Bevy Scripting

Although Bevy doesn't directly support scripting, efforts are underway to incorporate it. This crate represents an initial attempt to enable scripting within Bevy's existing framework. It's important to note that this is a work in progress and not yet optimized or complete. As Bevy evolves, significant changes to this API are anticipated.

## Why Use Scripts?

- Update your game mechanics without the need for re-compiling
- Encapsulating game logic in scripts paves way for modders to create custom content easilly
- Scripting game logic/UI in a simpler language broadens development accessibility to non-programmers on your team

## Features

- Script management via commands
- Hot loading
- Support for multiple scripting languages
- Extremely flexible bindings
    - Attach bindings to ANY reflect implementing types including foreign types.
    - Globals and bindings are set in one place and translated to all supported languages
    - Broad set of core Bevy bindings generated for you
- Dynamic systems & components registerable from scripts
    - Scripts can run in parallel to existing rust systems or other scripts      
- Customizable event driven communication between bevy and scripts (`on_update`, `on_init` etc..)
- Documentation generation

## Benchmarks
BMS applies continuous benchmarking and the latest benchmark results can be found over at [bencher.dev](https://bencher.dev/console/projects/bms/plots).

The tested scripts themselves are placed in the `assets/benchmarks` directory.

## Support

The languages currently supported are as follows:
|Language |
|----|
|Lua|
|Lua51|
|Lua52|
|Lua53|
|Lua54|
|Luajit|
|Luajit52|
|Luau|
|Rhai|
|~~Rune~~ temporarilly on hold[^1]|

## Documentation

For examples, installation and usage instructions see our shiny new [book](https://makspll.github.io/bevy_mod_scripting)

## Bevy Compatibility
BMS generates code based on each bevy release, which is why we pin the patch version of bevy too.

| bevy_mod_scripting  | bevy   |
|---------------------|--------|
| 0.14.0+             | 0.16.0 |
| 0.11.1+             | 0.15.3 |
| 0.9.5+              | 0.15.2 |
| 0.9.1+              | 0.15.1 |
| 0.9                 | 0.15.0 |
| 0.8                 | 0.15.0 |
| 0.7                 | 0.14   |
| 0.6                 | 0.13.1 |

## Supported Platforms

| platform | is supported? |
|----------|---------------|
| Windows  | yes |
| Linux    | yes |
| MacOS    | yes |
| WASM     | no, see [this issue](https://github.com/makspll/bevy_mod_scripting/issues/166) |

[^1]: Due to the recent re-write of the crate, documentation generation as well as rune support are temporarilly on hold. They will likely be re-implemented in the future.

[^2]: the coverage does not include generated bindings. 

[^3]: The crate strictly enforces no `unwrap`, `expect`, `panic` or `todo`'s via clippy lints.
