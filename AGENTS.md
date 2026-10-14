# bevy_mod_scripting (BMS)

Multi-language scripting for Bevy (Lua, Rhai, Rune). Scripts are Bevy assets; they access the ECS through reflection.

## Layout
* `src/` — facade crate re-exporting the others behind feature flags (language, per-Bevy-crate bindings).
* `crates/bevy_mod_scripting_{asset,script,core,world,bindings,bindings_domain,functions,display,derive}` — language-agnostic layers: assets/language ids, script lifecycle + callbacks + script systems (`core`), safe dynamic world access (`world`), value/reference/function/conversion machinery (`bindings`), registered global functions (`functions`).
* `crates/languages/*` — one crate per language backend, implementing the core plugin traits.
* `crates/bindings/*_bms_bindings` — **generated** per-Bevy-crate bindings. Don't hand-edit; regenerate via `codegen/`.
* `codegen/` — separate workspace (own pinned nightly) for `bevy_api_gen`, a cargo plugin that emits those bindings.
* `crates/ladfile*`, `crates/lad_backends/*` — LAD (Language Agnostic Declaration) JSON describing exported types/functions, plus backends (Lua language server files, mdbook docs).
* `crates/testing_crates`, `tests/` — script integration test harness; tests are driven by script files/scenarios.
* `xtask/` — all CI/dev tasks (build, check, test, docs); also a separate workspace. Prefer it over ad-hoc scripts.
* `docs/` — mdbook; `release-notes/`, per-crate `CHANGELOG.md` managed by release-plz.

## Big choices
* Scripts never touch Rust types directly: everything crosses the boundary as `ScriptValue`, with references to reflected data (`ReflectReference`) rather than copies.
* BMS uses its own reflection paths (`bindings/src/path`) instead of native Bevy ones, to support script-specific access semantics (e.g. 1-based indexing correction).
* World access is arbitrary at runtime, so `bevy_mod_scripting_world` enforces borrow safety dynamically via an access map + `WorldGuard`, rather than Bevy's static system params.
* Script lifecycle (load / attach / reload / unload) is a staged pipeline with per-language context management, in `core/src/pipeline`.
* Functions exposed to scripts are registered in a type-erased registry (`bindings/src/function`), not per-language; languages adapt to it. Operators/primitives live in `bindings_domain`.
* Functions available on every type's static reference (e.g. the shared `new` constructor) live on the `StaticTypeRef` namespace and are bound to the type at lookup (`lookup_type_function`), so languages only call `lookup_static_member`. Enum variants (`EnumVariantRef`) are resolved the same way, lazily.
* Every type known to scripts lives in a namespace: the crate defining it, unless overridden with `ScriptNamespace` type data (`script_bindings(namespace = ...)`); BMS' own types are in `bms`. Scripts register components and events only into `script.*` namespaces. `import_namespace(path)` returns a namespace's types (and exported functions), from the shared, lazily rebuilt `AppScriptTypeCatalogue` (`bindings/src/globals/catalogue.rs`). The flat `types` global is deprecated, and a snapshot taken at context creation.
* `export_namespace` exposes a callback of the calling script as a function in a `script.*` namespace; calls run through the callback runners in the exporting script. A script's exports and observers are removed when it unloads (`remove_script_registrations`).
* Script observers (`world.add_observer`) run their callbacks from a command through `AppScriptCallbackRunners::run_or_queue`, never re-entering a busy context.
* Feature flags select language and which Bevy crate bindings are compiled in; exactly one Lua variant must be chosen.

## Working here
* Builds are heavy; scope cargo to one crate (`-p`) and use `xtask` for anything broader.
* Lints are strict (see `clippy.toml`); `unwrap`/`expect` need justified `allow`s.

## Keeping notes in sync
* Crate-level `AGENTS.md` files (e.g. `crates/bevy_mod_scripting_core`, the test harness) hold area-specific patterns. If you make a major change to an area (architecture, conventions, how tests run), update its `AGENTS.md` and this file in the same change. Keep them short and high level.
