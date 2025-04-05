local fake_entity = Entity.from_raw(0)

assert_throws(function()
    world.insert_children(fake_entity, 1, {fake_entity})
end, "Missing or invalid entity")

local entity = world.spawn()
assert_throws(function()
    world.insert_children(entity, 1, {fake_entity})
end, "Missing or invalid entity")