# Script Systems

<div class="warning">
Script systems are an experimental feature
</div>

It's possible within BMS to inject new systems from within scripts themselves.

Systems introduced by scripts *can* run in parallel to other systems, and can be freely inserted between any other system, including other script systems.

BMS also provides utilities for visualising schedules using dot graphs, allowing low-effort modding frameworks for game authors.


## Schedules

Bevy doesn't support reflecting schedules, so BMS rolls it's own schedule registry resource: `AppScheduleRegistry`, which can be used to add any custom schedules you want to interact with. The default Bevy schedules will be pre-populated for you.

Once you've registered your schedule you will be able to interact with it in scripts like below:

```lua
local world = import_namespace("bms").world
local update_schedule = world.get_schedule_by_name("Update")
local systems = update:systems()
local system_with_name = update:get_system_by_name("my_system")
```

## Inserting Systems

To insert a system you will need to use `bms.system_builder` like below:

```lua
local bms = import_namespace("bms")
local attachment = bms.ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())
local system = bms.system_builder("my_system", attachment)
    :exclusive()
    :after(some_other_system)
    :before(another_system)
```

This will let you call `world.add_system` like so:

```lua
local world = import_namespace("bms").world
world.add_system(update_schedule,system)
```

## Ordering and sets

Besides systems (`after`/`before`), script systems can be ordered against:
- other script systems, by callback label and script, which must have been added before: `:after_script_system("on_physics", attachment)`
- sets: `:after_set(set)`, `:before_set(set)`, and put in sets: `:in_set(set)`

Scripts register their own sets into a `script.*` namespace, i.e. to group and order the systems of a mod. A set is removed from its namespace when its script unloads, registering it again gives the same set:

```lua
local ai = world.register_new_system_set("script.my_mod", "ai")
-- other scripts: import_namespace("script.my_mod").ai
```

Rust sets the game registers are members of their namespace, the BMS bindings register the main Bevy sets (i.e. `transform.TransformSystems.Propagate`, `time.TimeSystems`):

```rust,ignore
app.register_script_system_set("transform.TransformSystems", "Propagate", TransformSystems::Propagate);
```

```lua
local propagate = import_namespace("transform.TransformSystems").Propagate
builder:in_set(ai):after_set(propagate)
```

`world.get_system_set_by_name` finds either kind by its full name, i.e. `"transform.TransformSystems.Propagate"` or `"script.my_mod.ai"`.

## Run conditions

`:run_if` only runs the system if the condition holds, conditions are evaluated in Rust without calling the script. Every condition passed to `run_if` must hold:

```lua
local c = bms.conditions
builder:run_if(c.any_with_component(Enemy))
       :run_if(c.not_(c.resource_exists(Paused)))
       :run_if(c.on_timer(0.5))
```

Conditions: `resource_exists`, `resource_changed`, `resource_added`, `any_with_component`, `on_timer(seconds)`, `run_once` and the combinators `not_`, `and_`, `or_` (named so as `not`, `and`, `or` are Lua keywords).

## Lifetime

A script system is identified by its attachment and callback label: adding it again replaces the old one (with a warning). Script systems are removed when their script unloads or reloads, a reloading script adds them again in `on_script_loaded`.

Bevy cannot change a schedule while it runs, i.e. `PreUpdate` where scripts are loaded. Systems added to (or removed from) a running schedule are added (or removed) once it finishes, removed systems stop running right away. `world.add_system` then returns a system which can be used for ordering, but has no node in the schedule yet.

Errors in script systems are sent as `ScriptErrorMessage`s, like those of other callbacks.

## Parameters

The system builder allows script authors to parameterise systems, using `resource` and `query` functions.
The order in which those functions are called, will define the order in which arguments will be provided to the specified script callback.

For example:
```lua
local bms = import_namespace("bms")
local world = bms.world
bms.system_builder("my_system")
    :query(
        world.query()
            :component(ComponentA)
            :component(ComponentB)
            :with(ComponentC)
            :without(ComponentD)
    )
    :resource(ResourceA)
```

will create a system which calls the specified callback `my_system` with 2 arguments:
- The `ScriptQueryResult` for the first query
    - With `components` access to ComponentA and ComponentB
- The `ReflectReference` to `ResourceA`

`resource` and `component` give read and write access. Use `resource_read` and `component_read` for read only access, systems reading the same resources and components can run in parallel, while systems writing them cannot:

```lua
bms.system_builder("my_system", attachment)
    :query(world.query():component_read(ComponentA):component(ComponentB))
    :resource_read(Time)
```

## Exclusive systems

An exclusive system can be created using the `exclusive` function call on the system builder.

This allows the system to access everything as in a normal event handler.

Non-exclusive systems, will only be able to access the set of components and resources as parameterized when building the system. This is why we can run the system in parallel to other non-overlapping systems.

Components are only accessible on the entities matched by the queries declaring them, i.e. with `world.query():component(A):with(B)` the system can't access `A` on entities without `B`, since Bevy runs it in parallel with systems accessing `A` on those. Components only used as filters (`with`, `without`) can't be accessed.

Exclusive systems on the other hand, cannot run in parallel.


## Callback

The system injected will be similar to an event handler, however it will only trigger the specified script, and without any entity, in the first example you'd see the following lua callback:

```lua
function my_system()
    print("I am a dynamic system")
end
```

get triggered every update.
