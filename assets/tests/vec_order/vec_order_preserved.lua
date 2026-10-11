-- regression test for https://github.com/makspll/bevy_mod_scripting/issues/430
-- lists passed into bindings as `Vec<T>` must keep the order of the lua sequence

local function assert_order(input, label)
    local out = make_vec_i64(input)
    assert(#out == #input, label .. ": length mismatch, expected " .. #input .. " got " .. #out)
    for i = 1, #input do
        assert(out[i] == input[i], label .. ": index " .. i .. " expected " .. input[i] .. " got " .. tostring(out[i]))
    end
end

-- table constructors of every size, including even ones
for n = 1, 12 do
    local t = {}
    for i = 1, n do
        t[i] = i * 10
    end
    assert_order(t, "constructed by index, n=" .. n)
end

-- literal constructors
assert_order({ 1 }, "literal 1")
assert_order({ 1, 2 }, "literal 2")
assert_order({ 1, 2, 3 }, "literal 3")
assert_order({ 1, 2, 3, 4 }, "literal 4")
assert_order({ 1, 2, 3, 4, 5, 6 }, "literal 6")

-- appended via table.insert and t[#t + 1]
for n = 1, 12 do
    local t = {}
    for i = 1, n do
        table.insert(t, i)
    end
    assert_order(t, "table.insert, n=" .. n)

    local u = {}
    for i = 1, n do
        u[#u + 1] = i
    end
    assert_order(u, "append, n=" .. n)
end

-- filled in reverse, so elements land in the hash part first
for n = 1, 12 do
    local t = {}
    for i = n, 1, -1 do
        t[i] = i
    end
    assert_order(t, "reverse fill, n=" .. n)
end

-- elements removed from the front / back
do
    local t = { 1, 2, 3, 4, 5 }
    table.remove(t, 1)
    assert_order(t, "after remove front")
    table.remove(t)
    assert_order(t, "after remove back")
end

-- strings
local strs = make_vec_string({ "a", "b", "c", "d" })
assert(strs[1] == "a" and strs[2] == "b" and strs[3] == "c" and strs[4] == "d", "string order should be preserved")

-- empty
assert(#make_vec_i64({}) == 0, "empty list should stay empty")

-- lists returned from a binding (built on the rust side) must keep their order when passed back in
for n = 1, 12 do
    local input = {}
    for i = 1, n do
        input[i] = i * 10
    end
    local returned = make_vec_i64(input)
    assert_order(make_vec_i64(returned), "second round trip, n=" .. n)
end
