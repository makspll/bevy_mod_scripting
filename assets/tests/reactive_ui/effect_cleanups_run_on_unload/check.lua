local world = import_namespace("bms").world

function on_test()
    local CleanupRan = world.register_new_component("script.repro", "CleanupRan", true)
    local found = 0
    for _ in pairs(world.query():component(CleanupRan):build()) do found = found + 1 end
    if found == 0 then return "cleanup did not run" end
    return "cleanup ran"
end
