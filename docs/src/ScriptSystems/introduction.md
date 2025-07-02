# Script Systems

<div class="warning">
Script systems are an experimental feature
</div>

It's possible within BMS to inject new systems from within scripts themselves.

Systems introduced by scripts *can* run in parallel to other systems, and can be freely inserted between any other system, including other script systems.

BMS also provides utilities for visualising schedules using dot graphs, allowing low-effort modding frameworks for game authors.


## Schedules

Bevy doesn't support reflecting schedules, so BMS rolls its own schedule registry resource: `AppScheduleRegistry`, which can be used to add any custom schedules you want to interact with. The default Bevy schedules will be pre-populated for you.

Once you've registered your schedule you will be able to interact with it in scripts like below:

```lua
local update_schedule = world.get_schedule_by_name("Update")
local systems = update:systems()
local system_with_name = update:get_system_by_name("my_system")
```

## Inserting Systems

To insert a system you will need to use the `system_builder` global function like below:

```lua
local system = system_builder("my_system", script_id)
    :exclusive()
    :after(some_other_system)
    :before(another_system)
```

This will let you call `world.add_system` like so:

```lua
world.add_system(update_schedule,system)
```

<div class="warning">

If your event handler running the script is running in a certain schedule, that schedule will be temporarilly removed by Bevy. Meaning you won't be able to modify it from within the script in-flight.

</div>

## Parameters

The system builder allows script authors to parameterise systems, using `resource` and `query` functions.
The order in which those functions are called, will define the order in which arguments will be provided to the specified script callback.

For example:
```lua
system_builder("my_system")
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

## Exclusive systems

An exclusive system can be created using the `exclusive` function call on the system builder.

This allows the system to access everything as in a normal event handler.

Non-exclusive systems, will only be able to access the set of components and resources as parameterized when building the system. This is why we can run the system in parallel to other non-overlapping systems.

Exclusive systems on the other hand, cannot run in parallel.


## Callback

The system injected will be similar to an event handler, however it will only trigger the specified script, and without any entity, in the first example you'd see the following lua callback:

```lua
function my_system()
    print("I am a dynamic system")
end
```

get triggered every update.
