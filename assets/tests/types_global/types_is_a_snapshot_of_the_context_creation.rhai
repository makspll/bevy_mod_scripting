fn on_test() {
    assert(!("ScriptTypesEntryComp" in types), "not registered yet");
    world.register_new_component.call("script.test", "ScriptTypesEntryComp");
}

fn on_test_post_update() {
    // `types` is a snapshot taken when the context was created
    assert(!("ScriptTypesEntryComp" in types), "types is not resynced");
    assert("Transform" in types, "native types stay available");
}
