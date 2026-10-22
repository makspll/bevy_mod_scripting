-- small values (i.e. `Vec3`, `Entity`) are stored inline instead of allocated, scripts must not be able to tell
local world = import_namespace("bms").world
local Transform = import_namespace("transform").Transform
local Vec3 = import_namespace("math").Vec3

-- two variables holding the same value see each other's changes, like for allocated values
local a = Vec3.new(1, 2, 3)
local b = a
b.x = 5
assert(a.x == 5, "aliases share the value")

-- assigning a value copies it into the place, reads of fields are places
local t = Transform.from_xyz(1, 2, 3)
local v = Vec3.new(7, 8, 9)
t.translation = v
v.x = 10
assert(t.translation.x == 7, "assignment copies the value")
t.translation.y = 1
assert(t.translation.y == 1, "fields are places")
local translation = t.translation
translation.z = 4
assert(t.translation.z == 4, "a field read into a variable is still the place")

-- functions, operators and display work on values
assert(a:length() > 0, "methods")
assert((a + Vec3.new(1, 0, 0)).x == 6, "operators")
assert(a == Vec3.new(5, 2, 3), "equality")
assert(tostring(Vec3.new(1, 2, 3)) ~= nil, "display")

-- entities returned by functions and read from fields are equal
local parent = world.spawn()
local child = world.spawn()
world.push_children(parent, { child })
local child_of = world.get_component(child, world.get_type_by_name("bevy_ecs::hierarchy::ChildOf"))
assert(child_of[1] == parent, "field == value")
assert(parent == child_of[1], "value == field")
assert(child_of:parent() == parent, "function result == value")
assert(world.get_parent(child) == parent, "function results are equal")
