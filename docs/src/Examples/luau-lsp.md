## Running Luau 

Luau can be interesting for its typing system.
However, for it to be useful, it needs to be integrated both in the bevy lua runtime and also a language server much like luau-lsp.

This example will provide snippets to integrate `luau-lsp` and `bevy_mod_scripting` within VSCode

### Luau package import

The first step for packages to load each other is to enable the `unsafe_lua_modules` in `cargo.toml`, example:
```toml
[dependencies]
bevy_mod_scripting = { version = "0.13.0", features = ["luau","unsafe_lua_modules"] }
```

The second step to make luau`require("./source/to/package")` work within bevy, the `LUA_PATH` needs to be exported as an environment variable. This also probably needs to be initialized at runtime as the location of the bevy executable may change.
Here is an example snippet that updates the path at runtime.

```rust
fn main() {
    // Set the LUA_PATH env variable
    let mut assets_path = std::env::current_dir().expect("Failed to get current dir");
    assets_path.push("assets");

    let assets_str = assets_path
        .to_str()
        .expect("Failed to convert path to str")
        .replace("\\", "/");

    let luau_package_path = format!("{}{}", assets_str, "/?.luau");

    unsafe{
        env::set_var("LUA_PATH", luau_package_path);
    }

    let mut app = App::new();
    app.add_plugins(BMSPlugin);

    // More code adding luau scripts, callbacks and else.

    app.run();
}

```

Assuming a folder structure that is the following,
- assets
  - scenarios
    - scenario.luau
  - scripts
    - Template.luau

`scenario.luau` should be able to import Template.luau using the following lines:

```lua
local Template = require("./../scripts/Template")
```


### Luau-lsp

To have luau autocomplete and type check within VSCode install the Luau Language Server plugin.

To avoid excessive underlines within the luau-lsp, a `.luaurc` file needs to be added in the root of the project.

`.luaurc`
```json
{
	"languageMode": "strict",
	"lint": { "*": true, "LocalUnused": false, "FunctionUnused": false },
	"lintErrors": true,
	"globals": ["world" , "construct", "types", "package"]
}
```

Additionally, because luau-lsp is focused towards Roblox plugins, additional VSCode config should be inserted to avoid incorrect autocomplete.

`.vscode/settings.json`
```json
{
  "luau-lsp.platform.type": "standard",
  "luau-lsp.types.robloxSecurityLevel": "None",
  "luau-lsp.sourcemap.enabled": false,
  "luau-lsp.sourcemap.autogenerate": false
}
```