# bevy_mod_scripting (BMS)

Multi-language scripting for Bevy (Lua, Rune). Scripts are Bevy assets; they access the ECS through reflection.

## Layout
* `src/` — facade crate re-exporting the others behind feature flags (language, per-Bevy-crate bindings).
* `crates/bevy_mod_scripting_{asset,script,core,world,bindings,bindings_domain,functions,display,derive}` — language-agnostic layers: assets/language ids, script lifecycle + callbacks + script systems (`core`), safe dynamic world access (`world`), value/reference/function/conversion machinery (`bindings`), registered global functions (`functions`).
* `crates/languages/*` — one crate per language backend, implementing the core plugin traits.
* `crates/bindings/bevy_mod_scripting_bevy_<category>_bindings` — hand-written bevy bindings (see the "Abandoning Codegen" RFC), one crate per area of functionality (math, ecs, time, ui...), each behind a `bevy_<category>_bindings` feature and registered in `register_bevy_bindings`; conventions (what to bind, docs, `multi-lang-example` blocks, tests under `assets/tests/bindings/<category>/`) are in `docs/src/Development/writing-bindings.md`.
* `crates/ladfile*`, `crates/lad_backends/*` — LAD (Language Agnostic Declaration) JSON describing exported types/functions, plus backends (Lua language server files, mdbook docs).
* `crates/testing_crates`, `tests/` — script integration test harness; tests are driven by script files/scenarios.
* `xtask/` — all CI/dev tasks (build, check, test, docs); also a separate workspace. Prefer it over ad-hoc scripts.
* `docs/` — mdbook; `release-notes/`, per-crate `CHANGELOG.md` managed by release-plz.

## Big choices
* Scripts never touch Rust types directly: everything crosses the boundary as `ScriptValue`, with references to reflected data (`ReflectReference`) rather than copies. Values made for scripts (i.e. returned `V<T>`) are owned by the references to them (`ReflectAllocationId` wraps an `Arc`), there is no allocator or garbage collection.
* BMS uses its own reflection paths (`bindings/src/path`) instead of native Bevy ones, to support script-specific access semantics (e.g. 1-based indexing correction).
* World access is arbitrary at runtime, so `bevy_mod_scripting_world` enforces borrow safety dynamically via an access map + `WorldGuard`, rather than Bevy's static system params.
* Script lifecycle (load / attach / reload / unload) is a staged pipeline with per-language context management, in `core/src/pipeline`.
* Functions exposed to scripts are registered in a type-erased registry (`bindings/src/function`), not per-language; languages adapt to it. Operators/primitives live in `bindings_domain`.
* Languages dispatch every script operation through `bindings::dispatch` (indexing, operators, static members, printing, iteration) and only convert values and deliver method receivers; see `docs/src/Development/AddingLanguages/necessary-features.md`.
* Functions available on every type (e.g. the shared `new` constructor) are registered on `StaticTypeRef` (`FunctionScope::all_types()`) and bound to the type at lookup by currying a `ScriptValue::Type` (no allocation). Enum variants (`EnumVariantRef`) are resolved the same way, lazily. Signatures depending on a type use the generic markers of `function::generics` (`A`, `ComponentOf<A>`, `FieldsOf<A>`, ...); bounded markers bound the parameter (`A: Component`), and the LAD builder binds shared functions to each type satisfying the bounds.
* There are no script globals besides the builtins, hardcoded in `bindings::builtins` (`import_namespace`, `export_namespace`) plus the language's `register_callback`; the function registry is keyed by type only. `world` is the `bms.world` namespace value. Functions which don't belong to a type are members of a namespace (`#[script_bindings(namespace = "...")] impl NamespaceMembers`, stored in `AppScriptNamespaceMembers`), i.e. `import_namespace("bms").log_info`; the running script's entity and handle are `bms.current_entity()` / `bms.current_script()`, looked up only when called.
* Every type known to scripts lives in a namespace: the crate defining it, unless overridden with `ScriptNamespace` type data (`script_bindings(namespace = ...)`); BMS' own types are in `bms`. Scripts register components, events and system sets only into `script.*` namespaces; Rust system sets are namespace members (`ScriptSystemSetAppExt`). `import_namespace(path)` returns a namespace's types (and exported functions), from the shared, lazily rebuilt `AppScriptTypeCatalogue` (`bindings/src/globals/catalogue.rs`).
* `export_namespace` exposes a callback of the calling script as a function in a `script.*` namespace; calls run through the callback runners in the exporting script. A script's exports, system sets (from their namespace only) and observers are removed when it unloads (`remove_script_registrations`).
* Script observers (`world.add_observer`) run their callbacks from a command through `AppScriptCallbackRunners::run_or_queue`, never re-entering a busy context.
* The Rune backend (`crates/languages/bevy_mod_scripting_rune`) differs from Lua in how members are reached (names are registered up front, the context is rebuilt when new ones appear), has no top level code (`pub fn main()` runs on load), no shared globals (`state()`), and catches errors with `catch(|| ..)`; see its crate docs. Rune scripts (`.rn`) have tests next to the Lua ones, run with the `rune` feature.
* Feature flags select language and which Bevy crate bindings are compiled in; exactly one Lua variant must be chosen.

## Working here
* Builds are heavy; scope cargo to one crate (`-p`) and use `xtask` for anything broader.
* Lints are strict (see `clippy.toml`); `unwrap`/`expect` need justified `allow`s.

## Keeping notes in sync
* Crate-level `AGENTS.md` files (e.g. `crates/bevy_mod_scripting_core`, the test harness) hold area-specific patterns. If you make a major change to an area (architecture, conventions, how tests run), update its `AGENTS.md` and this file in the same change. Keep them short and high level.
