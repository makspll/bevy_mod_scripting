-- components and resources from `get_component`/`get_resource` are read and written in place; on LuaJIT they are views,
-- which go stale when the world changes structurally or the callback returns
local world = import_namespace("bms").world
local Transform = import_namespace("transform").Transform
local Vec3 = import_namespace("math").Vec3
local TransformType = world.get_type_by_name("bevy_transform::components::transform::Transform")
local ResourceType = world.get_type_by_name("test_utils::test_data::TestResourceWithVariousFields")
---@diagnostic disable-next-line: undefined-global
local is_jit = jit ~= nil

local function stale(f)
    local ok, err = pcall(f)
    -- other Lua versions return references, which stay valid
    if not is_jit then
        return assert(ok, tostring(err))
    end
    assert(not ok and tostring(err):find("stale view"), "expected a stale view, got: " .. tostring(err))
end

local a = world.spawn()
world.insert_component(a, TransformType, Transform.from_xyz(1, 2, 3))
local b = world.spawn()
world.insert_component(b, TransformType, Transform.from_xyz(4, 5, 6))

-- fields read and written in place, methods, and native fields written whole
local t = assert(world.get_component(a, TransformType))
assert(t.translation.x == 1, "translation.x: " .. tostring(t.translation.x))
t.translation = Vec3.new(7, 8, 9)
assert(assert(world.get_component(a, TransformType)).translation.y == 8, "the write is seen through a new view")
t:rotate_y(0)
assert_throws(function() t.translation.x = 5 end, ".*")
---@diagnostic disable-next-line: assign-type-mismatch
assert_throws(function() t.translation = 5 end, ".*")

-- scalar fields, with C keywords as names
local r = assert(world.get_resource(ResourceType))
r.float = 2.5
r.int = -3
r.bool = true
r.usize = 7
local r2 = assert(world.get_resource(ResourceType))
assert(r2.float == 2.5 and r2.int == -3 and r2.bool == true and r2.usize == 7, "resource fields")
if is_jit then
    assert_throws(function() r.int = 1.5 end, ".*int.*integer.*")
    ---@diagnostic disable-next-line: assign-type-mismatch
    assert_throws(function() r.bool = 1 end, ".*bool.*boolean.*")
end
-- other fields through reflection
r.string = "changed"
assert(r2.string == "changed", "slow field")

-- stale after structural changes: the column may move, a despawn swaps another row into this one
local tb = assert(world.get_component(b, TransformType))
world.despawn(a)
stale(function() return tb.translation end)
stale(function() tb.translation = Vec3.new(4, 5, 6) end)
assert(assert(world.get_component(b, TransformType)).translation.x == 4, "b kept its value")
local tb2 = assert(world.get_component(b, TransformType))
world.spawn()
stale(function() return tb2.translation end)
stale(function() return r.float end)

-- views can't be forged or changed
local tb3 = assert(world.get_component(b, TransformType))
---@diagnostic disable-next-line: inject-field
assert_throws(function() tb3._bms_p = 0 end, ".*")

function on_test()
    kept = assert(world.get_component(b, TransformType))
    assert(kept.translation.z == 6, "translation.z")
    return true
end

function on_test_post_update()
    -- another callback: the world may have changed in between
    stale(function() return kept.translation end)
    return true
end
