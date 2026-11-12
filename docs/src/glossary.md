# Glossary

The key concepts of BMS. Each entry's first paragraph is shown when hovering over the term elsewhere in the book.

<!-- Each entry is a `##` heading, an `aliases: a, b; not: phrase` comment listing the words to recognize in other pages
     (case insensitive, longest first wins, never inside the `not` phrases), then a one paragraph intuition.
     Keep that paragraph short: it is the hover card. Terms aren't linked on the pages their entry links to. -->

## Attachment
<!-- aliases: attachment, attachments, attached, attaching, attach, detached, detaching, detach -->

Where a script lives: pinned to an entity, or "static", pinned to nothing. A script file only runs once attached, and the same file attached twice runs twice, so the attachment, not the file, is what callbacks target.

In Rust it is a `ScriptAttachment`: `EntityScript(entity, handle)`, made by adding a `ScriptComponent`, or `StaticScript(handle)`, made with the `AttachScript` command. See [Managing Scripts](./Summary/managing-scripts.md).

## Binding
<!-- aliases: bindings, binding -->

A Rust function or type made available to scripts. Written once in Rust, bindings work in every supported language, and are documented in the generated reference.

Functions are registered with `#[script_bindings]` or a `NamespaceBuilder`, on a type or a namespace. See [Controlling Script Bindings](./Summary/controlling-script-bindings.md) and [Core Bindings](./ladfiles/bindings.lad.json).

## Callback
<!-- aliases: callbacks, callback -->

A function in a script that BMS calls by name, like an event listener. Everything a script does after loading happens inside a callback.

A callback is the function passed to `register_callback`, or else the top level function with that name. Callbacks are triggered by `event_handler` systems, `RunScriptCallback` commands, script systems, observers and other scripts. See [Running Scripts](./Summary/running-scripts.md) and [Callbacks](./Summary/callbacks.md).

## Callback label
<!-- aliases: callback labels, callback label -->

The name a callback is called by, as a Rust type, so Rust code can't misspell it.

Defined with `callback_labels!(OnEvent => "on_event")` and used by `event_handler::<OnEvent, _>` and `ScriptCallbackMessage`s. See [Running Scripts](./Summary/running-scripts.md).

## Context
<!-- aliases: script contexts, script context, contexts, context -->

The sandbox a script runs in, holding its global variables and functions, like a separate Lua state. Scripts in different contexts can't see each other's globals.

Contexts are created when scripts are attached, and removed with their last script. While one of its callbacks runs, a context is busy. See [Contexts](./Summary/contexts.md).

## Context policy
<!-- aliases: context policies, context policy -->

The rule deciding which scripts share a context: by default each entity-script pair gets its own, at the other end all scripts share one.

Set with `set_context_policy` on the scripting plugin. See [Contexts](./Summary/contexts.md#context-policies).

## Core callbacks
<!-- aliases: core callbacks, core callback -->

The callbacks BMS calls by itself around loading: `on_script_loaded`, `on_script_unloaded` and `on_script_reloaded`. Use them to set up a script and to carry state across hot reloads.

See [Core Callbacks](./ScriptingReference/core-callbacks.md).

## Event handler
<!-- aliases: event handlers, event handler -->

A Bevy system which delivers `ScriptCallbackMessage`s with one callback label to the scripts of one language, the usual way for Rust to call scripts every frame.

Added as `event_handler::<Label, Plugin>` in the schedule you want the callbacks to run in. See [Running Scripts](./Summary/running-scripts.md#event-handlers).

## Hot reloading
<!-- aliases: hot reloading, hot reloads, hot reload, hot-reloading, hot-reloadable, reloading, reloaded, reloads, reload -->

Editing a script file while the game runs, and having the new code take effect without a restart. The script's top level code runs again in its existing context.

Needs Bevy's `file_watcher` feature. `on_script_unloaded` can return state which `on_script_reloaded` receives. See [Managing Scripts](./Summary/managing-scripts.md#hot-reloading).

## Initializer
<!-- aliases: context initializers, context initializer, pre handling initializers, pre handling initializer, runtime initializers, runtime initializer, initializers, initializer -->

A Rust function run on a context, or runtime, to customize it, i.e. to set globals before the script's code runs.

Context initializers run when a context is created or reloaded, pre handling initializers before every callback, runtime initializers once. See [Customizing Contexts](./Summary/customizing-script-contexts.md).

## LAD file
<!-- aliases: LAD files, LAD file, LADfiles, LADfile, ladfiles, ladfile -->

A language agnostic JSON description of every type and function scripts can use. It drives the generated reference pages and the Lua language server definitions.

See [Editor Support](./ScriptTooling/introduction.md).

## Namespace
<!-- aliases: namespaces, namespace -->

A named group of types and functions which scripts import, i.e. `import_namespace("math")`. Types are not globals, so namespaces keep same named types apart.

Rust types live in the namespace of their crate or binding category, scripts register their own components and events into `script.*` namespaces. See [Namespaces](./ScriptingReference/namespaces.md).

## Reflect reference
<!-- aliases: reflect references, reflect reference -->

A script's pointer into the Bevy world: to a component, a resource, or a value BMS allocated. Reading or writing its fields reads or writes the real value, through reflection.

Its base is a component, a resource, an allocation or an asset. In Rust it is a `ReflectReference`, received as `R<T>`, `M<T>` or `V<T>` in binding signatures. See [References](./ScriptingReference/references.md).

## Runtime
<!-- aliases: runtimes, runtime; not: at runtime, runtime error, runtime errors -->

The engine state shared by all contexts of one language, if the language has one. Contexts are the sandboxes, the runtime is what they run on. Lua has no separate runtime, Rune does.

Customized with runtime initializers. See [Customizing Contexts](./Summary/customizing-script-contexts.md#runtime-initializers).

## Script
<!-- aliases: script assets, script asset, script files, script file -->

A Lua or Rune source file, loaded as a Bevy asset (`ScriptAsset`). Loading it runs nothing: it runs once attached.

See [Managing Scripts](./Summary/managing-scripts.md).

## Script pipeline
<!-- aliases: script pipeline, pipeline -->

The machinery which turns attachments, detachments and modified assets into loaded, reloaded or unloaded scripts, running the core callbacks along the way.

Runs in `PreUpdate`, and immediately for `AttachScript` / `DetachScript`. See [Script Pipeline](./ScriptPipeline/pipeline.md).

## Script system
<!-- aliases: script systems, script system -->

A Bevy system added by a script, which calls one of the script's callbacks each time its schedule runs, with the queries and resources it asked for.

Built with `bms.system_builder` and added with `world.add_system`. See [Script Systems](./ScriptSystems/introduction.md).

## Script value
<!-- aliases: script values, script value -->

The language neutral value passed between Rust and scripts: integers, strings, lists, functions, references and so on. Each language converts it to and from its own values.

In Rust it is `ScriptValue`, converted with `FromScript` and `IntoScript`. See [Controlling Script Bindings](./Summary/controlling-script-bindings.md#dynamic-functions).

## Static script
<!-- aliases: static scripts, static script -->

A script attached to nothing rather than to an entity, i.e. game wide logic.

Attached and detached with the `AttachScript` and `DetachScript` commands. See [Managing Scripts](./Summary/managing-scripts.md#static-scripts).
