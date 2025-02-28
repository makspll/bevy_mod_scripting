let runs = [];

fn on_test() {
    let post_update_schedule = world.get_schedule_by_name.call("PostUpdate");
    let test_system = post_update_schedule.get_system_by_name.call("on_test_post_update");

    let builder_after = system_builder.call("custom_system_after", script_id).after.call(test_system);
    let system_after = world.add_system.call(post_update_schedule, builder_after);
    
    let builder_before = system_builder.call("custom_system_before", script_id).before.call(test_system);
    let system_before = world.add_system.call(post_update_schedule, builder_before);
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

fn on_test_last() {
    assert(runs.len() == 3, "Expected 3 runs, got: " + runs.len().to_string());
    assert(runs[0] == "custom_system_before", "Expected custom_system_before to run first, got: " + runs[0]);
    assert(runs[1] == "on_test_post_update", "Expected on_test_post_update to run second, got: " + runs[1]);
    assert(runs[2] == "custom_system_after", "Expected custom_system_after to run third, got: " + runs[2]);
}