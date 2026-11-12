# Examples

Rust examples live in the [`examples`](https://github.com/makspll/bevy_mod_scripting/tree/main/examples) folder:

| Example | Shows |
|---|---|
| `game_of_life` | A full app driven by Lua or Rune scripts, with callbacks, static scripts and custom bindings |
| `runscript` | Running script files given on the command line |
| `script_loading` | Waiting for scripts to load with `ScriptPipelineState` |
| `custom_conversions` | Hand-written `FromScript` / `IntoScript` conversions |
| `docgen` | Generating LAD files and Lua language server definitions |

Run one with `cargo run --example <name> --features lua54` (`docgen` needs `lua_language_server_files`).

For example scripts, see the regression test suite in [`assets/tests`](https://github.com/makspll/bevy_mod_scripting/tree/main/assets/tests), which covers every scripting function in Lua and Rune.