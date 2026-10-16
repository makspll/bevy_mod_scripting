local world = import_namespace("bms").world
local entity = world.spawn()
local _type = world.get_type_by_name('test_utils::test_data::CompWithFromWorld')

assert_throws(function()
    world.add_default_component(entity, _type)
end, 'Missing type data ReflectComponent for type: .*CompWithFromWorld.*')