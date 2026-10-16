local world = import_namespace("bms").world
local component = world.get_type_by_name("test_utils::test_data::TestComponent")
local entity = world.spawn()
local retrieved = world.get_component(entity, component)

assert(retrieved == nil, "Component found")