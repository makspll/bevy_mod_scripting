local world = import_namespace("bms").world
local entity_a = world.spawn()
local entity_b = world.spawn()
local entity_c = world.spawn()
local entity_d = world._get_entity_with_test_component("CompWithFromWorldAndComponentData")

local component_with = world.get_type_by_name("test_utils::test_data::CompWithFromWorldAndComponentData")
local component_without = world.get_type_by_name("test_utils::test_data::CompWithDefaultAndComponentData")

world.add_default_component(entity_a, component_with)
world.add_default_component(entity_b, component_with)
world.add_default_component(entity_c, component_with)

world.add_default_component(entity_b, component_without)

local found_entities = {}
for i, result in pairs(world.query():component(component_with):without(component_without):build()) do
    table.insert(found_entities, result[1])
end

assert(#found_entities == 3, "Expected 3 entities, got " .. #found_entities)

expected_entities = {
    entity_d,
    entity_a,
    entity_c,
}

for i, entity in ipairs(found_entities) do
    assert(entity:index() == expected_entities[i]:index(),
        "Expected entity " .. expected_entities[i]:index() .. " but got " .. entity:index())
end
