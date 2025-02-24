local entity = world.spawn()
local _type = world.get_type_by_name('CompWithDefault')
local entity_with_component = world._get_entity_with_test_component('CompWithDefault')
local existing_component = world.get_component(entity_with_component, _type)

assert_throws(function()
    world.insert_component(entity, _type, existing_component)
end, "Missing type data ReflectComponent for type: .*CompWithDefault.*")
