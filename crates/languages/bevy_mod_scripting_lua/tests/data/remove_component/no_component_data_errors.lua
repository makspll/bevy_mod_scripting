
local entity = _get_entity_with_test_component("CompWithDefault")
local component = world.get_type_by_name("CompWithDefault")

assert_throws(function ()
    world.remove_component(entity, component)
end, "Missing type data ReflectComponent for type: .*CompWithDefault.*")
