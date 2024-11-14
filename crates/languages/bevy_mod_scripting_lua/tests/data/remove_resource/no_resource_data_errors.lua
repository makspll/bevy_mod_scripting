
local type = _get_mock_type()

assert_throws(function ()
    world:remove_resource(type)
end, "Does not have ReflectResource data registered")
