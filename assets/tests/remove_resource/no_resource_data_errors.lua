local type = world._get_mock_resource_type()

assert_throws(function()
    world.remove_resource(type)
end, "Missing type data ReflectResource for type: Unregistered Type.*")
