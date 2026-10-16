let bms = import_namespace.call("bms");
let world = bms.world;
fn on_test() {
    let ScriptAttachment = import_namespace.call("bms").ScriptAttachment;
    let post_update_schedule = world.get_schedule_by_name.call("PostUpdate");
    
    let script_attachment = ScriptAttachment.new_entity_script.call(bms.current_entity.call(), bms.current_script.call());
    let test_system = post_update_schedule.get_system_by_name.call("on_test_post_update");
    
    let system_a = world.add_system.call(
        post_update_schedule,
        bms.system_builder.call("custom_system_a", script_attachment)
            .after.call(test_system)
    );
        
    let system_b = world.add_system.call(
        post_update_schedule,
        bms.system_builder.call("custom_system_b", script_attachment)
            .after.call(test_system)
    );
        
    // the new systems both run after the existing one, but run in parallel (no ordering between them)
    world._assert_system_runs_before.call(post_update_schedule, "on_test_post_update", "custom_system_a");
    world._assert_system_runs_before.call(post_update_schedule, "on_test_post_update", "custom_system_b");
    world._assert_systems_run_in_parallel.call(post_update_schedule, "custom_system_a", "custom_system_b");
}
