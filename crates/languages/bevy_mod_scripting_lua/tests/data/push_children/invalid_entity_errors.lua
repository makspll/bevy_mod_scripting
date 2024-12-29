local fake_entity = Entity.from_raw(9999)

assert_throws(function()
    world:push_children(fake_entity, {fake_entity})
end, "Missing or invalid entity")

local entity = world:spawn()
assert_throws(function()
    world:push_children(entity, {fake_entity})
end, "Missing or invalid entity")