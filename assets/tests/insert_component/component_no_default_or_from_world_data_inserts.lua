local world = import_namespace("bms").world
local entity = world.spawn()
local type = world.get_type_by_name('test_utils::test_data::TestComponent')
local entity_with_component = world._get_entity_with_test_component('TestComponent')
local existing_component = world.get_component(entity_with_component, type)

assert(world.has_component(entity, type) == false, 'Expected entity to not have component before adding, test invalid')
world.insert_component(entity, type, existing_component)
assert(world.has_component(entity, type) == true, 'Expected entity to have component after adding')
