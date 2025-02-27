local entity = world.spawn()
local _type = world.get_type_by_name('CompWithFromWorld')

assert_throws(function()
    world.add_default_component(entity, _type)
end, 'Missing type data ReflectComponent for type: .*CompWithFromWorld.*')