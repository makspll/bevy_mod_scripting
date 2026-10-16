local world = import_namespace("bms").world
local entity_with_component =world._get_entity_with_test_component("TestComponent")

local component_type = world.get_type_by_name("test_utils::test_data::TestComponent")

function bench()
    local strings = world.get_component(entity_with_component, component_type).strings
end