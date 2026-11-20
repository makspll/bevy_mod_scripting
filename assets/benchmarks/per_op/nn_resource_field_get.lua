-- 100 operations per call: the cost of one is (this - `empty loop`) / 100
local world = import_namespace("bms").world
local r = world.get_resource(world.get_type_by_name("test_utils::test_data::TestResourceWithVariousFields"))

function bench()
    for i = 1, 100 do
        local x = r.float
    end
end
