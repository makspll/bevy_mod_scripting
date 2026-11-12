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

## Where to start

| You are | Read |
|---|---|
| Adding scripting to a Bevy game | [Installation](./Summary/installation.md), then the [User Guide](./Summary/managing-scripts.md) |
| Writing scripts or mods | The [Scripting Reference](./ScriptingReference/introduction.md) |
| Upgrading BMS | The [Release Notes](./ReleaseNotes/guides.md) |
| Contributing to BMS | [Developing BMS](./Development/introduction.md) |
