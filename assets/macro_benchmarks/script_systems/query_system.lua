-- a non-exclusive script system querying two components with an empty callback, measures building and passing the payload
local bms = import_namespace("bms")
local world = bms.world
local ComponentA = world.get_type_by_name("test_utils::test_data::CompWithFromWorldAndComponentData")
local ComponentB = world.get_type_by_name("test_utils::test_data::CompWithDefaultAndComponentData")

function on_script_loaded()
    world.add_system(
        world.get_schedule_by_name("Update"),
        bms.system_builder("bench_system", bms.ScriptAttachment.new_static_script(bms.current_script()))
        :query(world.query():component(ComponentA):component_read(ComponentB))
    )
end

function bench_system(results)
end
