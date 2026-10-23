-- parameters declared with `resource_read` / `component_read` are declared as reads to bevy,
-- so the system may read but must not write them
local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
local world = bms.world
runs = {}
local ResourceType = world.get_type_by_name("test_utils::test_data::TestResourceWithVariousFields")
local ComponentA = world.get_type_by_name("test_utils::test_data::CompWithFromWorldAndComponentData")

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())

    local entity = world.spawn()
    world.add_default_component(entity, ComponentA)

    world.add_system(
        post_update_schedule,
        bms.system_builder("my_reading_system", script_attachment)
        :resource_read(ResourceType)
        :query(world.query():component_read(ComponentA))
    )

    return true
end

function my_reading_system(resource, query)
    runs[#runs + 1] = "my_reading_system"
    local ok, err = pcall(check_accesses, resource, query)
    runs[#runs + 1] = ok and "finished" or tostring(err)
end

function check_accesses(resource, query)
    assert(resource.string == "Initial Value", "Expected 'Initial Value', got: " .. resource.string)
    assert(#query >= 1, "Expected results, got: " .. #query)
    local component = query[1]:components()[1]
    assert(type(component[1]) == "string", "Expected to read the component, got: " .. tostring(component[1]))

    local ok, err = pcall(function() resource.string = "changed" end)
    assert(not ok and tostring(err):find("annot claim access"), "resource write: " .. tostring(err))
    ok, err = pcall(function() component[1] = "changed" end)
    assert(not ok and tostring(err):find("annot claim access"), "component write: " .. tostring(err))
end

function on_test_post_update()
    return true
end

function on_test_last()
    assert(#runs == 2 and runs[2] == "finished", "the system did not finish, runs: " .. table.concat(runs, ","))
    return true
end
