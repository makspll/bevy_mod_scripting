let runs = [];

fn on_test() {
    let post_update_schedule = world.get_schedule_by_name.call("PostUpdate");
    let test_system = post_update_schedule.get_system_by_name.call("on_test_post_update");
    let script_attachment = ScriptAttachment.new_entity_script.call(entity, script_asset);

    let builder_after = system_builder.call("custom_system_after", script_attachment)
        .after.call(test_system);
    let system_after = world.add_system.call(post_update_schedule, builder_after);
    
    let builder_before = system_builder.call("custom_system_before", script_attachment)
        .before.call(test_system);
    let system_before = world.add_system.call(post_update_schedule, builder_before);

    let builder_between = system_builder.call("custom_system_between", script_attachment)
        .after.call(test_system)
        .before.call(system_after);

    let system_between = world.add_system.call(post_update_schedule, builder_between);
}

fn custom_system_before() {
    print("custom_system_before");
    runs.push("custom_system_before");
}

fn on_test_post_update() {
    print("on_test_post_update");
    runs.push("on_test_post_update");
}

fn custom_system_after() {
    print("custom_system_after");
    runs.push("custom_system_after");
}

fn custom_system_between() {
    print("custom_system_between");
    runs.push("custom_system_between");
}

fn on_test_last() {
    assert(runs.len() == 4, "Expected 4 runs, got: " + runs.len().to_string());
    assert(runs[0] == "custom_system_before", "Expected custom_system_before to run first, got: " + runs[0]);
    assert(runs[1] == "on_test_post_update", "Expected on_test_post_update to run second, got: " + runs[1]);
    assert(runs[2] == "custom_system_between", "Expected custom_system_between to run third, got: " + runs[2]);
    assert(runs[3] == "custom_system_after", "Expected custom_system_after to run third, got: " + runs[3]);
}