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
- All script bindings managed in one place (`ScriptDynamicFunctionRegistry`)
- Customizable event driven communication between bevy and scripts (`on_update`, `on_init` etc..)
- Automatically generated bevy bindings
- ~Documentation generation~ temporarilly on hold

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
|Rune|

## Documentation

For examples, installation and usage instructions see our shiny new [book](https://makspll.github.io/bevy_mod_scripting)
