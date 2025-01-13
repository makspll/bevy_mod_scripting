# Setup

this crate contains a work in progress `xtask` setup which in theory should allow you to setup everything you need for local development by running:

```sh
cargo xtask init
```

However at the moment it does not generate IDE specific files for you, so you will need to do that manually.

## VScode

For vscode you will want to enter the following into your `settings.json`

```json
{
    "rust-analyzer.rustc.source": "discover",
    "rust-analyzer.linkedProjects": [
        // "./crates/bevy_api_gen/Cargo.toml", uncommenting this is currently not fully supported with xtask + vscode, rust analyzer bugs out a lot sadly
        "Cargo.toml",
    ],
    "rust-analyzer.check.invocationStrategy": "once",
    "rust-analyzer.check.overrideCommand": [
        "/absolute-path-to-this-project/bevy_mod_scripting/check.sh"
    ],
    "rust-analyzer.cargo.buildScripts.overrideCommand": [
        "/absolute-path-to-this-project/bevy_mod_scripting/check.sh"
    ],
}
```

If you are on windows you might need to create an equivalent `check.exe` to run `cargo xtask check --ide-mode` in the root directory of this workspace.
