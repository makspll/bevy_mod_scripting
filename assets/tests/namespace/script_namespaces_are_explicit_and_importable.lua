local function error_of(f, ...)
    local ok, err = pcall(f, ...)
    assert(not ok, "expected an error")
    return tostring(err)
end

function on_test()
    -- namespaces must be passed, and be script namespaces
    local err = error_of(world.register_new_component, "game", "Enemy")
    assert(err:find("must start with `script.`", 1, true), err)
    assert(error_of(world.register_new_component, "bevy_transform", "Enemy"):find("script.", 1, true))

    -- the same name can be used in different namespaces
    local ai_enemy = world.register_new_component("script.game.ai", "Enemy")
    local ui_enemy = world.register_new_component("script.game.ui", "Enemy")
    assert(ai_enemy:namespace() == "script.game.ai")
    assert(ui_enemy:namespace() == "script.game.ui")

    -- but not twice in the same one, which the error says
    local dup = error_of(world.register_new_component, "script.game.ai", "Enemy")
    assert(dup:find("script.game.ai.Enemy", 1, true) and dup:find("allow_existing", 1, true), dup)

    -- they can be imported
    local ai = import_namespace("script.game.ai")
    assert(ai.Enemy ~= nil, "registered components are importable")
    local entity = world.spawn()
    world.add_default_component(entity, ai.Enemy)
    assert(world.has_component(entity, ai_enemy))
    assert(not world.has_component(entity, ui_enemy))
end
