function Test()
    local entity = world:spawn()
    local _type = world:get_type_by_name('CompWithFromWorld')

    world:add_default_component(entity, _type)
end 

local success,err = pcall(Test)
assert(not success, 'No error thrown')
assert(string.find(tostring(err), 'Does not have ReflectComponent'), 'Error contains wrong message: ' .. tostring(err))