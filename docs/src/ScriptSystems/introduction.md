# Script Systems

<div class="warning">
Script systems are an experimental feature
</div>

It's possible within BMS to inject new systems from scripts themselves, although the support is currently limited.

Systems introduced by scripts cannot run in parallel to other systems, but can be freely inserted between any other rust system (not script systems at the moment) and into any schedule.


## Schedules

Bevy doesn't support reflecting schedules, so BMS rolls it's own schedule registry resource: `AppScheduleRegistry`, which can be used to add any custom schedules you want to interact with. The default Bevy schedules will be pre-populated for you.

Once you've registered your schedule you will be able to interact with it in scripts like below:

```lua
local update_schedule = world.get_schedule_by_name("Update")
local systems = update:systems()
local system_with_name = update:get_system_by_name("my_system")
```

## Inserting Systems

To insert a system wou will need to use the `system_builder` global function like below:

```lua
local system = system_builder("my_system", script_id)
    :after(some_Other_system)
    :before(another_system)
```

This will let you call `world.add_system` like so:

```lua
world.add_system(update_schedule,system)
```

<div class="warning">

If your event handler running the script is running in a certain schedule, that schedule will be temporarilly removed by Bevy. Meaning you won't be able to modify it from within the script in-flight.

</div>

## Dynamic system

The system injected will be similar to an event handler, however it will only trigger the specified script, and without any entity, in the above example you'd see the following lua callback:

```lua
function my_system()
    print("I am a dynamic system")
end
```

get triggered every update.
