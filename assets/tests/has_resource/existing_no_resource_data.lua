local world = import_namespace("bms").world
-- without `ReflectResource` the declarations don't know it is a resource
local component = world.get_type_by_name("test_utils::test_data::ResourceWithDefault") --[[@as bms.ScriptResourceRegistration<test_utils.ResourceWithDefault>]]
assert(world.has_resource(component) == true, "Resource was not found")