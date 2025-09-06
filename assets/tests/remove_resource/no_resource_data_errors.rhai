
let type = world._get_mock_resource_type.call();

assert_throws(||{
    world.remove_resource.call(type)
}, "Missing type data ReflectResource for type: Unregistered Type.*");
