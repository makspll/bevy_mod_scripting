let bms = import_namespace.call("bms");
let world = bms.world;
let runs = [];

fn on_test() {
    let ScriptAttachment = import_namespace.call("bms").ScriptAttachment;
    let post_update_schedule = world.get_schedule_by_name.call("PostUpdate");
    let script_attachment = ScriptAttachment.new_entity_script.call(bms.current_entity.call(), bms.current_script.call());
    world.add_system.call(post_update_schedule, bms.system_builder.call("my_non_exclusive_system", script_attachment));
    return true;
}

fn my_non_exclusive_system() {
    print("my_non_exclusive_system");
    runs.push("my_non_exclusive_system");

    let ResourceType = world.get_type_by_name.call("test_utils::test_data::TestResource");
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