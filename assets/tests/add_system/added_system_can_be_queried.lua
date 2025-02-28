local post_update_schedule = world.get_schedule_by_name("PostUpdate")

local system = world.add_system(
    post_update_schedule,
    system_builder("custom_system", script_id)
)

local retrieved_system = post_update_schedule:get_system_by_name("custom_system")
assert(retrieved_system:path() == "custom_system", "System path is not correct got: ".. retrieved_system:path())