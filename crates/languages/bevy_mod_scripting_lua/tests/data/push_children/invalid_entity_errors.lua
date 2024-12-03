local fake_entity = Entity.from_raw(9999)

assert_throws(function()
    world:push_children(fake_entity, {fake_entity})
end, "The parent Entity does not exist")

local entity = world:spawn()
assert_throws(function()
    world:push_children(entity, {fake_entity})
end, "the Entity does not exist")