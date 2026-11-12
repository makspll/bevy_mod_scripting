# Installation

## Cargo

Add BMS to your `Cargo.toml` with a language feature:

```toml
bevy_mod_scripting = { version = "0.24", features = ["lua54"] }
```

Each BMS release supports one Bevy version, see the [compatibility table](https://github.com/makspll/bevy_mod_scripting#bevy-compatibility).

## Language Features

| Language | Feature Flag |
| ---- | ---- |
| Lua 5.1 | `lua51` |
| Lua 5.2 | `lua52` |
| Lua 5.3 | `lua53` |
| Lua 5.4 | `lua54` |
| LuaJIT | `luajit` |
| LuaJIT 5.2 | `luajit52` |
| Luau | `luau` |
| Rune | `rune` |

Only one Lua feature can be enabled at a time. Lua and Rune can be enabled together.

## Bevy Plugin

Add the BMS plugin group to your app:

```rust,ignore
app.add_plugins(BMSPlugin);
```

Each plugin in the group can be replaced with a configured one using `BMSPlugin.set(MySubPlugin)`.
Next, [load and attach your first script](./managing-scripts.md).

## Extra Features

The default features are `core_functions` and the ecs, input, math, time, transform, color and window bindings. Disable default features if you only need BMS for script lifecycle management and want to register the bindings yourself.

| Feature | Description |
| ---- | ---- |
| `core_functions` | The core functions, i.e. `world` and the other ways to interact with Bevy via reflection |
| `bevy_<crate>_bindings` | Bindings for a Bevy crate, i.e. `bevy_math_bindings`, `bevy_transform_bindings`. See the `[features]` of the [Cargo manifest](https://github.com/makspll/bevy_mod_scripting/blob/main/Cargo.toml) for the full list |
| `lua_language_server_files` | Generates Lua language server definitions, see [Editor Support](../ScriptTooling/introduction.md) |
| `mlua_async` | Enables `mlua/async` |
| `mlua_serialize` | Enables `mlua/serialize` |
| `mlua_macros` | Enables `mlua/macros` |
| `unsafe_lua_modules` | Allows loading native modules via `require` in Lua |
| `profile_with_tracy` | Adds tracy profiling spans |

## Mobile

CI type-checks `bevy_mod_scripting` for `aarch64-linux-android` and `aarch64-apple-ios`; nothing is run on a device. Both need the target's C toolchain (Android NDK, Xcode) as the Lua runtimes are built from source, and Android needs a `bevy` activity feature in your app (e.g. `bevy/android-native-activity`).

| Language | Android | iOS |
| ---- | ---- | ---- |
| Lua 5.1, 5.2, 5.3 | yes | no: the Lua sources call `system`, which the iOS SDK marks unavailable (fixed upstream in Lua 5.4) |
| Lua 5.4 | yes | yes |
| LuaJIT, LuaJIT 5.2 | yes | no: `luajit-src` can't build for iOS, which also forbids JIT |
| Luau | yes | yes |
| Rune | yes | yes |

`unsafe_lua_modules` loads native libraries at runtime, which iOS restricts.
