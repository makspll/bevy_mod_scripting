---
applyTo: **
---

# Repository Instructions

## High-Level Structure

### Core Components
- **bevy_mod_scripting** - Main workspace crate that re-exports key functionality.
- **bevy_mod_scripting_core** - Core framework with language-agnostic scripting functionality
- **bevy_mod_scripting_asset** - Handles script assets, loading, and language detection
- **bevy_mod_scripting_derive** - Procedural macros for generating bindings
- **bevy_mod_scripting_functions** - Core Bevy function bindings, and plugin managing various binding crates via features.

### Language Implementations
- **bevy_mod_scripting_lua** - Lua language implementation
- **bevy_mod_scripting_rhai** - Rhai language implementation

### Binding Crates
- Multiple **bevy_*_bms_bindings** crates (e.g., transform, asset, pbr, etc.) that provide specific Bevy module bindings
- These are automatically generated via the `/codegen` tools, per bevy release

### Development Tools
- **xtask** - Custom task runner for development workflows
- **codegen** - Code generation tools for generating bindings from Bevy's API
- **docs** - Documentation built with mdbook
- **docs/src/ReleaseNotes** - Release notes and migration guides

## Key Features

1. **Script Management** - Loading, hot reloading, and lifecycle management via Bevy's asset system
2. **Flexible Bindings** - Attach bindings to any Reflect-implementing types
3. **Dynamic Systems** - Create ECS systems from scripts that run in parallel
4. **Component Creation** - Register and use components from scripts
5. **Multiple Language Support** - Currently supports Lua and Rhai


## Xtask Commands
- `cargo xtask check` - Builds the project and runs clippy
