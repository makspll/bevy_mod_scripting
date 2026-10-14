local function error_of(f, ...)
    local ok, err = pcall(f, ...)
    assert(not ok, "expected an error")
    return tostring(err)
end

function on_test()
    local bevy_transform = import_namespace("bevy_transform")
    assert(bevy_transform.Transform ~= nil, "Transform should be in bevy_transform")
    assert(bevy_transform.Transform:namespace() == "bevy_transform")
    -- other crates' types are not in it
    assert(bevy_transform.Entity == nil)

    local bms = import_namespace("bms")
    assert(bms.ScriptAttachment ~= nil)

    -- imported types are the same ones as looked up by name
    assert(bevy_transform.Transform:type_name() == world.get_type_by_name("bevy_transform::components::transform::Transform"):type_name())

    -- missing namespaces say so, with hints
    local typo = error_of(import_namespace, "bevy_transfrom")
    assert(typo:find('namespace "bevy_transfrom" does not exist', 1, true), typo)
    assert(typo:find("Did you mean", 1, true) and typo:find("bevy_transform", 1, true), typo)
    assert(typo:find("registered in the type registry", 1, true), typo)

    -- script namespaces get their own hint
    local script = error_of(import_namespace, "script.game.ai")
    assert(script:find('namespace "script.game.ai" does not exist', 1, true), script)
    assert(script:find("loaded script", 1, true), script)
end
