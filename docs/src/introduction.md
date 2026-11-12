<div class="bms-hero">
  <img src="logo.svg" alt="">
  <h1>Bevy Mod Scripting</h1>
  <p class="bms-tagline">Script your Bevy game: hot-reloadable Lua &amp; Rune with full ECS access</p>
  <div class="bms-hero-links">
    <a href="Summary/installation.html">Install</a>
    <a href="Summary/managing-scripts.html">User Guide</a>
    <a href="ScriptingReference/introduction.html">Scripting Reference</a>
    <a href="https://github.com/makspll/bevy_mod_scripting">GitHub</a>
  </div>
</div>

BMS (`bevy_mod_scripting`) runs Lua and Rune scripts inside a Bevy app. Scripts reach the world through reflection, so any `Reflect` type, including ones from other crates, can be used from scripts without writing glue code.

- Update game logic without recompiling, with hot reloading of script assets
- Let modders add content through the same bindings your own scripts use
- Bindings are written once in Rust and work in every supported language
- Scripts can add their own systems, components and events
- Generated docs and Lua language server definitions for your bindings

## How a script runs

A script only runs code at the points below: when it is attached (or reloaded, or detached), and whenever one of its callbacks is invoked.

```mermaid
flowchart TD
    ATTACH["<b>Attach</b><br/>ScriptComponent inserted<br/>AttachScript command<br/>ScriptLifecycleMessage::Attached"]
    ATTACH --> LOAD["<b>Load</b>, in the script pipeline<br/>wait for the asset<br/>assign a context, run context initializers<br/>run the top level code<br/>on_script_loaded()"]
    LOAD --> LIVE(["Attached"])
    LIVE <-- "invoked by name" --> CB["<b>Callbacks</b><br/>Rust: ScriptCallbackMessage + event_handler<br/>Rust: RunScriptCallback command<br/>Scripts: bms.run_script_callback, exports<br/>Script systems, each schedule run<br/>Observers, on triggered events"]
    LIVE -- "asset modified" --> RELOAD["<b>Reload</b><br/>on_script_unloaded() returns state<br/>run the top level code again<br/>on_script_loaded()<br/>on_script_reloaded(state)"]
    RELOAD --> LIVE
    LIVE -- "ScriptComponent removed,<br/>DetachScript, Detached message" --> UNLOAD["<b>Unload</b><br/>on_script_unloaded()<br/>context removed with its last script"]
```

- **Loading** a script asset runs nothing, only [attaching](./Summary/managing-scripts.md) it does. The attachment (entity + script, or a static script) is the script's identity.
- Scripts sharing a [context](./Summary/contexts.md) share globals, the context policy decides which do.
- **Callbacks** are functions the script defines, run by name: the one passed to `register_callback` if any, else the top level function. Pre handling initializers run before each. See [Running Scripts](./Summary/running-scripts.md), [Callbacks](./Summary/callbacks.md), the [core callbacks](./ScriptingReference/core-callbacks.md) and [Calling Other Scripts](./ScriptingReference/calling-other-scripts.md).
- The [script pipeline](./ScriptPipeline/pipeline.md) has the details on ordering and budgeting.

## Where to start

| You are | Read |
|---|---|
| Adding scripting to a Bevy game | [Installation](./Summary/installation.md), then the [User Guide](./Summary/managing-scripts.md) |
| Writing scripts or mods | The [Scripting Reference](./ScriptingReference/introduction.md) |
| Upgrading BMS | The [Release Notes](./ReleaseNotes/guides.md) |
| Contributing to BMS | [Developing BMS](./Development/introduction.md) |
