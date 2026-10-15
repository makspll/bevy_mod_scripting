# Calling Other Scripts

Scripts can run callbacks of other scripts, in any scripting language, using two functions:

| Function | When the callback runs | Returns |
|---|---|---|
| `run_script_callback(label, target, ...args)` | Immediately, before the call returns | What the callback returned |
| `queue_script_callback(label, target, ...args)` | As soon as the current callback finishes, in the same frame | Nothing |

```lua
local bms = import_namespace("bms")
local ScriptAttachment, Recipients = bms.ScriptAttachment, bms.Recipients
local enemy_script = ScriptAttachment.new_entity_script(enemy, script_asset)

-- runs `take_damage(10, "fire")` in the enemy's script right away
local remaining_health = run_script_callback("take_damage", enemy_script, 10, "fire")

-- runs `on_hit()` in every script once this callback is done
queue_script_callback("on_hit", Recipients.all_scripts())
```

```rhai
let ScriptAttachment = import_namespace.call("bms").ScriptAttachment;
let Recipients = import_namespace.call("bms").Recipients;
let enemy_script = ScriptAttachment.new_entity_script.call(enemy, script_asset);

let remaining_health = run_script_callback.call("take_damage", enemy_script, 10, "fire");

queue_script_callback.call("on_hit", Recipients.all_scripts.call());
```

## Targets

The target is either a single script:

* `ScriptAttachment.new_static_script(script)`
* `ScriptAttachment.new_entity_script(entity, script)`

or a set of `Recipients`:

* `Recipients.all_scripts()`: every loaded script, once per script even if scripts share a context
* `Recipients.all_contexts()`: every loaded context once
* `Recipients.static_script(script)`
* `Recipients.entity_script(entity, script)`

For a single script, `run_script_callback` returns the callback's value. For recipients it returns a list with the value of each script which ran the callback.
Scripts which are still loading or unloading are skipped when targeting many scripts, while targeting a specific script which is not loaded is an error.

## Errors

* `run_script_callback` raises the callback's error in the calling script, so it can be handled like any other error. For recipients, the first error is raised after all scripts ran the callback.
* Errors of queued callbacks cannot reach the script which queued them, they are reported like other script errors (logged by default).

## Which scripts can be called immediately

While a callback runs, its script context is busy, so `run_script_callback` cannot run a callback in a busy context. This includes:

* the calling script itself (call the function directly instead),
* scripts sharing a context with the calling script,
* scripts further up the current chain of callbacks, e.g. A calls B, which calls A again.

These calls return an error instead. `queue_script_callback` works for all of them, since queued callbacks only run once the current callback has finished.

Whether scripts share a context depends on the context policy of the scripting plugin:

| Context policy | Scripts sharing a context |
|---|---|
| `ContextPolicy::per_entity_and_script()` (default) | None |
| `ContextPolicy::per_entity()` | All scripts attached to the same entity |
| `ContextPolicy::per_script()` | All attachments of the same script |
| `ContextPolicy::shared()` | All scripts |

## Queued callbacks

* Queued callbacks run in the order they were queued, and can queue further callbacks, which run in the same go.
* They never run in the middle of the callback which queued them, even if it keeps using the world afterwards.
* To stop callbacks from queueing each other endlessly, at most 1024 queued callbacks run in one go. Any beyond that run later, and an error is reported.

## Script systems

Script systems running in parallel with other systems (i.e. not marked `exclusive`) cannot use `run_script_callback`, as another system might be running the target script at the same time.
Callbacks they queue run at the end of the frame instead, in the `ScriptingSystemSet::RunQueuedCallbacks` system set in the `Last` schedule.

## Limits

* Callbacks can be nested at most 64 levels deep, i.e. a callback running another callback which runs another one and so on.
