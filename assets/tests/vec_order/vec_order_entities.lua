-- regression test for https://github.com/makspll/bevy_mod_scripting/issues/430
-- lists of entities passed into bindings (`Vec<Val<Entity>>`) must keep their order

local function spawn_n(n)
    local entities = {}
    for i = 1, n do
        entities[i] = world.spawn()
    end
    return entities
end

local function assert_same_entities(expected, actual, label)
    assert(#actual == #expected, label .. ": length mismatch, expected " .. #expected .. " got " .. #actual)
    for i = 1, #expected do
        assert(
            actual[i]:index() == expected[i]:index(),
            label .. ": entity at position " .. i .. " is wrong"
        )
    end
end

for n = 1, 10 do
    local entities = spawn_n(n)

    -- identity round trip
    assert_same_entities(entities, make_vec_entity(entities), "round trip, n=" .. n)

    -- push_children then get_children
    local parent = world.spawn()
    world.push_children(parent, entities)
    assert_same_entities(entities, world.get_children(parent), "push_children, n=" .. n)

    -- entities in reverse order must stay reversed
    local reversed = {}
    for i = n, 1, -1 do
        reversed[#reversed + 1] = entities[i]
    end
    assert_same_entities(reversed, make_vec_entity(reversed), "reversed round trip, n=" .. n)

    -- insert_children in the middle keeps the given order
    local parent2 = world.spawn()
    local first = world.spawn()
    local last = world.spawn()
    world.push_children(parent2, { first, last })
    world.insert_children(parent2, 2, entities)
    local expected = { first }
    for _, e in ipairs(entities) do
        expected[#expected + 1] = e
    end
    expected[#expected + 1] = last
    assert_same_entities(expected, world.get_children(parent2), "insert_children, n=" .. n)
end

-- tables obtained from the bindings can be passed straight back in
do
    local parent = world.spawn()
    world.push_children(parent, spawn_n(4))
    local children = world.get_children(parent)
    assert_same_entities(children, make_vec_entity(children), "binding-returned list round trip")
end
