## LADfiles
BMS provides a custom Language Agnostic Declaration format (LAD), which can be used to integrate with other tooling.

If you use `#[script_bindings]` macros, these files will magically contain your code comments, bindings as well as everything else required to drive other powerful integrations.

The declaration file is generated from the type registry and serves as a light abstraction layer simplifying the traversal of all
available types, namespaces and functions.

You can customize how / where and if these files are stored using the main BMS plugin group:
```rust,ignore
    app.add_plugins(BMSPlugin.set::<ScriptingFilesGenerationPlugin>(
        ScriptingFilesGenerationPlugin::new(
            true, // enabled, you can use a compilation feature to disable this here
            PathBuf::from("assets").join("definitions"),
            Some(PathBuf::from("bindings.lad.json")), // do also save the ladfile itself
            "Core BMS framework bindings",
            true,
            true,
        ),
    ));
```

This plugin is only available when one of the sub features (like `lua_language_server_files`) mentioned in this chapter is enabled.


You might not want to run this pipeline in your final binary, but rather bundle some of the generated files into some sort of development pack for modding. You can use compiler flags like `#[cfg(not(debug_assertions))]` to disable ladfile generation at runtime, or simply disable the lower level features within BMS to avoid compiling related dependencies too.

## Lua Language Server

<div class="warning">
    This feature is in early stages, the definitions provide 90% of the way there, but there are rough edges that will need to be worked out
</div>

[Luals](https://github.com/LuaLS/lua-language-server) or LLS, is an open source language server which integrates with many IDE's.

It is powered by lua specific annotation or definition files which BMS can generate directly from its own LADfiles.

To enable this simply enable the `lua_language_server_files` feature, and a `bindings.d.lua` definition file, plus a `bindings.<namespace>.d.lua` file per namespace, will be generated in the LADfile output directory in the `Startup` schedule.

Script writers can then use this generated file by pointing their `.luarc.json` file to these definitions:
```json
{
    "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
    "workspace.library": [
        "assets/definitions"
    ],
    "runtime.version": "Lua 5.4",
    "hint.enable": false,
    "workspace.preloadFileSize": 10000
}
```

The generated definitions file is large (over a megabyte with the default bindings), and the language server skips Lua files bigger than `workspace.preloadFileSize` (in KB, 500 by default).
If completions for BMS types are missing, i.e. `Vec3.` suggests nothing, raise this limit above the size of the generated file and restart the language server.

