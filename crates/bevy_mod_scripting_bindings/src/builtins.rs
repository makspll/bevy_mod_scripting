//! The builtins, the only globals of scripts: `import_namespace`, `export_namespace` and `register_callback`.
//!
//! They are not registered anywhere. Languages bind [`builtins`] into every context, and implement `register_callback` themselves.
//! Everything else is imported from a namespace, i.e. `import_namespace("bms").world`.

use std::collections::VecDeque;

use bevy_ecs::schedule::SystemSet;
use bevy_platform::collections::HashMap;
use bevy_system_reflection::ReflectSystemSet;

use crate::{
    AppScriptNamespaceMembers, DynamicScriptFunction, FunctionCallContext, FunctionInfo,
    FunctionScope, GetFunctionInfo, InteropError, ScriptNamedSet, ScriptTypeCatalogue, ScriptValue,
    WorldExtensions, WorldGuard, function::from::V, function::into::IntoScript,
    function::script_function::documented_function, script_path,
};

const IMPORT_NAMESPACE_DOCS: &str = r#"Imports a namespace, returning a table of the types, functions, values, events, system sets and exports in it by name.

Bevy types with bindings live in a namespace per category (`math`, `ecs`, `transform`, `ui`, ...), BMS' own types and functions in `bms`,
any other type in the namespace named after the crate defining it. Namespaces defined by scripts start with `script.`.

```lua
local bms = import_namespace("bms")
local Vec3 = import_namespace("math").Vec3
bms.world.spawn()
```

Fails with a list of similar namespaces if the namespace does not exist.

Arguments:
* `path`: the name of the namespace to import.
Returns:
* `namespace`: a table of the contents of the namespace keyed by name."#;

const EXPORT_NAMESPACE_DOCS: &str = r#"Exports a callback of the calling script as a function in a script namespace, so that any script (in any language) can call it.

The callback is either a function registered with `register_callback`, or a top level function with that name.
Calls run in the exporting script and return what the callback returns.

```lua
register_callback("spawn_enemy_impl", function(health) ... end)
export_namespace("script.game", "spawn_enemy", "spawn_enemy_impl")
-- in another script, once the first one has run
import_namespace("script.game").spawn_enemy(5)
```

Arguments:
* `namespace`: the script namespace to export into, it must start with `script.`.
* `name`: the name of the function within the namespace.
* `callback`: the label of the callback of this script to export.
Returns:
* `result`: Nothing if the function was exported."#;

const REGISTER_CALLBACK_DOCS: &str = r#"Registers a function as the handler of a callback, taking precedence over a top level function of that name.

```lua
register_callback("on_script_unloaded", function() print("unloading!") end)
```

Arguments:
* `callback`: the callback label to register the function against.
* `function`: the function handling the callback."#;

/// Imports a namespace, see the docs of the builtin.
pub fn import_namespace(world: WorldGuard, path: &str) -> Result<ScriptValue, InteropError> {
    profiling::function_scope!("import_namespace");
    let catalogue = world.type_catalogue().clone().get(world.clone())?;
    let types = catalogue.namespace_view(path);
    let (exports, events, system_sets) = {
        let registry = world.component_registry();
        let registry = registry.read();
        let exports = registry
            .iter_exports_in(path)
            .map(|e| e.name.clone())
            .collect::<Vec<_>>();
        let events = registry
            .iter_events_in(path)
            .map(|(name, event)| (name.to_owned(), event.clone()))
            .collect::<Vec<_>>();
        let system_sets = registry
            .iter_system_sets_in(path)
            .map(|(set_path, name)| (name.to_owned(), set_path.to_owned()))
            .collect::<Vec<_>>();
        (exports, events, system_sets)
    };
    let members = world
        .with_resource(|members: &AppScriptNamespaceMembers| members.read().get(path).cloned())
        .ok()
        .flatten();
    if types.is_none()
        && exports.is_empty()
        && events.is_empty()
        && system_sets.is_empty()
        && members.is_none()
    {
        return Err(namespace_not_found(&catalogue, &world, path));
    }

    let mut table = HashMap::default();
    for (name, function) in members.iter().flat_map(|m| &m.functions) {
        table.insert(name.to_string(), ScriptValue::Function(function.clone()));
    }
    for (name, value) in members.iter().flat_map(|m| &m.values) {
        table.insert(name.to_string(), (value.maker)(world.clone())?);
    }
    for (name, registration) in types.into_iter().flatten() {
        table.insert(name, registration.into_script(world.clone())?);
    }
    for (name, event) in events {
        table.insert(name, V(event).into_script(world.clone())?);
    }
    for (name, set_path) in system_sets {
        let set = ReflectSystemSet::from_interned(ScriptNamedSet(set_path).intern());
        table.insert(name, V(set).into_script(world.clone())?);
    }
    for name in exports {
        let function = exported_function(script_path(path, &name));
        table.insert(name, ScriptValue::Function(function));
    }
    Ok(ScriptValue::Map(table))
}

/// Exports a callback of the running script, see the docs of the builtin.
pub fn export_namespace(
    world: WorldGuard,
    namespace: &str,
    name: &str,
    callback: String,
) -> Result<(), InteropError> {
    profiling::function_scope!("export_namespace");
    let owner = world.current_attachment().0.ok_or_else(|| {
        InteropError::str(
            "Cannot export a function outside of a script callback, as it needs a script to run it in",
        )
    })?;
    world.register_script_export(namespace, name, callback, owner)
}

/// The builtins languages bind into every context, besides their own `register_callback`.
pub fn builtins() -> [(&'static str, DynamicScriptFunction); 2] {
    [
        (
            "import_namespace",
            documented_function(
                FunctionScope::Builtin,
                "import_namespace".into(),
                |ctxt: FunctionCallContext, path: String| import_namespace(ctxt.world()?, &path),
                Some(IMPORT_NAMESPACE_DOCS),
                Some(&["path"]),
            ),
        ),
        (
            "export_namespace",
            documented_function(
                FunctionScope::Builtin,
                "export_namespace".into(),
                |ctxt: FunctionCallContext, namespace: String, name: String, callback: String| {
                    export_namespace(ctxt.world()?, &namespace, &name, callback)
                },
                Some(EXPORT_NAMESPACE_DOCS),
                Some(&["namespace", "name", "callback"]),
            ),
        ),
    ]
}

/// The signatures and docs of all builtins, including `register_callback`, which languages implement.
pub fn builtin_docs() -> Vec<FunctionInfo> {
    let register_callback = (|_: String, _: DynamicScriptFunction| {})
        .get_function_info("register_callback".into(), FunctionScope::Builtin)
        .with_docs(REGISTER_CALLBACK_DOCS)
        .with_arg_names(&["callback", "function"]);
    builtins()
        .into_iter()
        .map(|(_, function)| (*function.info).clone())
        .chain([register_callback])
        .collect()
}

/// A function calling the function with the given full path exported by a script, in the script that exported it.
///
/// The export is looked up on every call, so it picks up replacements, and fails clearly if the export is gone.
fn exported_function(path: String) -> DynamicScriptFunction {
    (move |ctxt: FunctionCallContext, args: VecDeque<ScriptValue>| -> ScriptValue {
        match ctxt
            .world()
            .and_then(|world| world.call_script_export(&path, Vec::from(args)))
        {
            Ok(value) => value,
            Err(error) => ScriptValue::Error(error),
        }
    })
    .into()
}

/// Builds the error for importing a namespace which does not exist.
fn namespace_not_found(
    catalogue: &ScriptTypeCatalogue,
    world: &WorldGuard,
    path: &str,
) -> InteropError {
    let mut message = format!("namespace \"{path}\" does not exist.");
    if path.starts_with("script.") {
        message.push_str(
            " Script namespaces only exist once a loaded script has registered or exported something into them, \
            make sure that script has loaded and run before importing it.",
        );
    } else {
        message.push_str(
            " Bevy types are grouped by category (i.e. `math`, `transform`), BMS' types live in `bms` and other types in the namespace named after their crate. \
            Namespaces only contain types registered in the type registry: check that the bindings plugin of the category is added and the type is registered.",
        );
    }
    let mut namespaces = world
        .component_registry()
        .read()
        .export_and_event_namespaces()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if let Ok(members) = world.with_resource(|members: &AppScriptNamespaceMembers| {
        members
            .read()
            .iter()
            .map(|(namespace, _)| namespace.to_string())
            .collect::<Vec<_>>()
    }) {
        namespaces.extend(members);
    }
    let similar = catalogue.similar_namespaces_with(path, &namespaces);
    if !similar.is_empty() {
        message.push_str(&format!(" Did you mean: {}?", similar.join(", ")));
    }
    InteropError::string(message)
}
