# Contexts

Each script runs in a context: the environment holding its global variables and functions. Which scripts share a context is decided by the **context policy** of the scripting plugin. By default each entity-script pair gets its own context, isolated from all other scripts.

## Context policies

| Policy | Scripts sharing a context |
|---|---|
| `ContextPolicy::per_entity_and_script()` (default) | None, the most isolated |
| `ContextPolicy::per_entity()` | All scripts attached to the same entity |
| `ContextPolicy::per_script()` | All attachments of the same script, i.e. one script on many entities |
| `ContextPolicy::shared()` | All scripts, there is only one context |

Set the policy on the scripting plugin:

```rust,ignore
app.add_plugins(LuaScriptingPlugin::default().set_context_policy(
    ContextPolicy::shared(),
));
```

Sharing a context makes it easy to share data between scripts, at the cost of scripts being able to interfere with each other.

> [!WARNING]
> In a shared context, scripts overwrite each other's top level functions when loaded. Use [registered callbacks](./callbacks.md#registered-callbacks) so each callback reaches the right script.

A script's context is busy while one of its callbacks runs, which limits which scripts it can [call immediately](../ScriptingReference/calling-other-scripts.md#which-scripts-can-be-called-immediately).

## Custom policies

A policy is a list of rules, tried in order until one applies. `per_script()` could be written as:

```rust,ignore
let policy = ContextPolicy {
    priorities: vec![Arc::new(ContextRule::Script), Arc::new(ContextRule::Shared)],
};
```

Each rule maps the script's attachment to a `ContextKey`, and scripts with the same key share a context:

```rust,ignore
pub struct ContextKey {
    pub entity: Option<Entity>,
    pub script: Option<AssetId<ScriptAsset>>,
}
```

So the policy reads: if there is a script, the key is just the script. Failing that, `ContextRule::Shared` always applies, returning the empty key.

For entirely custom rules, implement `ContextKeySelector`, or pass a closure `Fn(&ScriptAttachment) -> Option<ContextKey>`.

To change what is in a context, see [Customizing Contexts](./customizing-script-contexts.md).
