local world = import_namespace("bms").world
local entity = world.spawn()
local type = world.get_type_by_name('test_utils::test_data::TestComponent')

assert_throws(function()
    world.add_default_component(entity, type)

end, "Missing type data ReflectDefault or ReflectFromWorld for type: .*TestComponent.*")