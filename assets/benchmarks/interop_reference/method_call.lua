-- #bench interop reference/method call
-- a vector reached through a component: the reflection path, with world access claims
local world = import_namespace("bms").world
local Transform = import_namespace("transform").Transform
local transform_type = world.get_type_by_name("bevy_transform::components::transform::Transform")
local entity = world.spawn()
world.insert_component(entity, transform_type, Transform.from_xyz(1.0, 2.0, 3.0))
local t = world.get_component(entity, transform_type)

function bench()
    local l = t.translation:length()
end
