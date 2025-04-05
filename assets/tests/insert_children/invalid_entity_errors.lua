local fake_entity = Entity.from_raw(0)
local fake_entity_valid = Entity.from_raw(9999)

assert_throws(function()
    world.insert_children(fake_entity_valid, 1, {fake_entity_valid})
end, "Missing or invalid entity")

local entity = world.spawn()
assert_throws(function()
    world.insert_children(entity, 1, {fake_entity})
end, "Are you trying to use an entity in a callback in which it's unavailable?")