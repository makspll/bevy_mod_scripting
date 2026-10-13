fn on_test() {
    assert(!("ScriptTypesEntryComp" in types), "not registered yet");
    world.register_new_component.call("script.test", "ScriptTypesEntryComp");
}

fn on_test_post_update() {
    assert("ScriptTypesEntryComp" in types, "script component should appear in types");
    assert("Transform" in types, "native types stay available");

    let entity = world.spawn_.call();
    world.add_default_component.call(entity, types.ScriptTypesEntryComp);
    assert(world.has_component.call(entity, types.ScriptTypesEntryComp));
}
