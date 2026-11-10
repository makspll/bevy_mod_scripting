local world = import_namespace("bms").world
-- registered without `ReflectResource`, so the declarations don't know it's a resource
local resource = world.get_type_by_name("test_utils::test_data::ResourceWithDefault") --[[@as bms.ScriptResourceRegistration<test_utils.ResourceWithDefault>]]

local retrieved = world.get_resource(resource)
assert(retrieved ~= nil, "Resource should exist")
assert(retrieved[1] == "Initial Value", "Resource should have default value but got: " .. retrieved[1])
