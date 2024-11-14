local entity = world:spawn()
local type = world:get_type_by_name('TestComponent')

assert_throws(function()
    world:add_default_component(entity, type)

end, "Does not have ReflectDefault or ReflectFromWorld data registered")