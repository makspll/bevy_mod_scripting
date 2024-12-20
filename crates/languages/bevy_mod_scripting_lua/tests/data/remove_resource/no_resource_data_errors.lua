
local type = _get_mock_type()

assert_throws(function ()
    world:remove_resource(type)
end, "Missing type data ReflectResource for type: Unregistered TypeId.*")
