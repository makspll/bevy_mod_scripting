let runs = [];

fn on_test() {
    let post_update_schedule = world.get_schedule_by_name.call("PostUpdate");
    world.add_system.call(post_update_schedule, system_builder.call("my_non_exclusive_system", script_id));
    return true;
}

fn my_non_exclusive_system() {
    print("my_non_exclusive_system");
    runs.push("my_non_exclusive_system");

    let ResourceType = world.get_type_by_name.call("TestResource");
    assert_throws(|| {
        let res = world.get_resource.call(ResourceType);
        let blah = res.blahblah
    }, ".*annot claim access to.*");
}

fn on_test_post_update() {
    return true;
}

fn on_test_last() {
    assert(runs.len == 1, "Expected 1 runs, got: " + runs.len);
    return true;
}