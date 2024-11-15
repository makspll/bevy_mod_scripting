local fake_entity = Entity.from_raw(9999)

assert_throws(function()
    world:insert_children(fake_entity, 1, {fake_entity})
end, "parent Entity does not exist")

local entity = world:spawn()
assert_throws(function()
    world:insert_children(entity, 1, {fake_entity})
end, "the Entity does not exist")