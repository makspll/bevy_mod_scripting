local fake_entity = _make_invalid_entity()

assert_throws(function()
    world.push_children(fake_entity, { fake_entity })
end, "Missing or invalid entity")

local entity = world.spawn()
assert_throws(function()
    world.push_children(entity, { fake_entity })
end, "Missing or invalid entity")
