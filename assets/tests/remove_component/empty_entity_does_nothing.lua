local world = import_namespace("bms").world
local entity = world.spawn()
local type = world.get_type_by_name('test_utils::test_data::TestComponent')

world.remove_component(entity, type)
world.remove_component(entity, type)