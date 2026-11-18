local world = import_namespace("bms").world

function on_test()
    local CleanupRan = world.register_new_component("script.repro", "CleanupRan", true)
    if #world.query():component(CleanupRan):build() == 0 then return "cleanup did not run" end
    return "cleanup ran"
end
