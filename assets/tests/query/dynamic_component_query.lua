local entity_a = world.spawn()
local NewComponent = world.register_new_component("NewComponent")

world.add_default_component(entity_a, NewComponent)

local found_entities = {}
for i, result in pairs(world.query():component(NewComponent):build()) do
    table.insert(found_entities, result:entity())
end

assert(#found_entities == 1, "Expected 1 entities, got " .. #found_entities)

expected_entities = {
    entity_a
}

for i, entity in ipairs(found_entities) do
    assert(entity:index():index() == expected_entities[i]:index():index(),
        "Expected entity " .. expected_entities[i]:index():index() .. " but got " .. entity:index():index())
end
