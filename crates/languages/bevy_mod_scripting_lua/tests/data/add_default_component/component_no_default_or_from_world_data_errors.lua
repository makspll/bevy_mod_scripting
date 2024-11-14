function Test()
    local entity = world:spawn()
    local type = world:get_type_by_name('TestComponent')
    world:add_default_component(entity, type)
end

local success,err = pcall(Test)
assert(not success, 'No error thrown')
assert(string.find(tostring(err), 'Does not have ReflectDefault or ReflectFromWorld'), 'Error contains wrong message: ' .. tostring(err))