-- create a dynamic component with n depth of fields
local Component = world.register_new_component("DeepComponent");
local new_entity = world.spawn();

reseed()

local keys = {
    "_0",
    "my_key",
    "longish_key"
}

-- Recursively build a single nested table using the pre-selected step_keys.
local function make_path(step_keys, index)
    if index > #step_keys then
        return {}
    end
    local key = step_keys[index]
    return { [key] = make_path(step_keys, index + 1) }
end


local current_reference = nil
local steps = 10
local step_keys = {}
function pre_bench()
    -- Choose keys for every step.
    for i = 1, steps do
        local key = keys[random_int(1, #keys)]
        step_keys[i] = key
    end

    -- Build the nested table.
    local instance = make_path(step_keys, 1)
    world.remove_component(new_entity, Component)
    world.insert_component(new_entity, Component, construct(types.DynamicComponent, {
        data = instance
    }))
    current_reference = world.get_component(new_entity, Component)
end

function bench()
    -- reference into random key into current_reference steps times
    local reference = current_reference.data
    local current_step = 1
    while current_step <= steps do
        reference = reference[step_keys[current_step]]
        current_step = current_step + 1
    end
end