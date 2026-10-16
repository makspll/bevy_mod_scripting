local world = import_namespace("bms").world
local type = world.get_type_by_name("test_utils::test_data::TestResource")

world.remove_resource(type)
world.remove_resource(type)