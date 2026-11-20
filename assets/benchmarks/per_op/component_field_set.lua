-- 100 operations per call: the cost of one is (this - `empty loop`) / 100
local world = import_namespace("bms").world
local Transform = import_namespace("transform").Transform
local transform_type = world.get_type_by_name("bevy_transform::components::transform::Transform")
local entity = world.spawn()
world.insert_component(entity, transform_type, Transform.from_xyz(1.0, 2.0, 3.0))
local t = world.get_component(entity, transform_type)

function bench()
    for i = 1, 100 do
        t.translation.x = i
    end
end
