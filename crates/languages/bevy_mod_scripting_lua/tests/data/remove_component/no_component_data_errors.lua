
local entity = _get_entity_with_test_component("CompWithDefault")
local component = world:get_type_by_name("CompWithDefault")

assert_throws(function ()
    world:remove_component(entity, component)
end, "Does not have ReflectComponent data registered")
