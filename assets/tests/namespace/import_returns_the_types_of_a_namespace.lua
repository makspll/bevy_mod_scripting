local function error_of(f, ...)
    local ok, err = pcall(f, ...)
    assert(not ok, "expected an error")
    return tostring(err)
end

function on_test()
    local transform = import_namespace("transform")
    assert(transform.Transform ~= nil, "Transform should be in transform")
    assert(transform.Transform:namespace() == "transform")
    -- other categories' types are not in it
    assert(transform.Entity == nil)

    local bms = import_namespace("bms")
    local world = bms.world
    assert(bms.ScriptAttachment ~= nil)

    -- imported types are the same ones as looked up by name
    assert(transform.Transform:type_name() == world.get_type_by_name("bevy_transform::components::transform::Transform"):type_name())

    -- imported types give access to their static functions and enum variants
    assert(transform.Transform.from_xyz(1, 2, 3).translation.y == 2)
    local SimpleEnum = import_namespace("test_utils").SimpleEnum
    assert(SimpleEnum.Unit ~= nil, "variants are reachable")

    -- and work wherever a type is expected
    local entity = world.spawn()
    world.add_default_component(entity, transform.Transform)
    assert(world.has_component(entity, transform.Transform))

    -- and have the functions of their kind of type
    local Transform = transform.Transform
    assert(Transform.is_on(entity) and Transform.get_on(entity).translation.x == 0)
    Transform.insert_on(entity, Transform.from_xyz(1, 2, 3))
    assert(Transform.get_on(entity).translation.y == 2)
    Transform.remove_from(entity)
    assert(not Transform.is_on(entity) and Transform.get_on(entity) == nil)
    Transform.add_default_on(entity)
    assert(Transform.is_on(entity))
    local TestResource = import_namespace("test_utils").TestResource
    assert(TestResource.exists() == (TestResource.fetch() ~= nil))
    local not_a_resource = error_of(Transform.fetch)
    assert(not_a_resource:find("ReflectResource", 1, true), not_a_resource)

    -- missing namespaces say so, with hints
    local typo = error_of(import_namespace, "transfrom")
    assert(typo:find('namespace "transfrom" does not exist', 1, true), typo)
    assert(typo:find("Did you mean", 1, true) and typo:find("transform", 1, true), typo)
    assert(typo:find("registered in the type registry", 1, true), typo)

    -- script namespaces get their own hint
    local script = error_of(import_namespace, "script.game.ai")
    assert(script:find('namespace "script.game.ai" does not exist', 1, true), script)
    assert(script:find("loaded script", 1, true), script)
end
