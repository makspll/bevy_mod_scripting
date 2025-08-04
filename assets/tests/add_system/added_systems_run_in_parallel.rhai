fn on_test() {
    let post_update_schedule = world.get_schedule_by_name.call("PostUpdate");
    
    let script_attachment = ScriptAttachment.new_entity_script.call(entity, script_id);
    let test_system = post_update_schedule.get_system_by_name.call("on_test_post_update");
    
    let system_a = world.add_system.call(
        post_update_schedule,
        system_builder.call("custom_system_a", script_attachment)
            .after.call(test_system)
    );
        
    let system_b = world.add_system.call(
        post_update_schedule,
        system_builder.call("custom_system_b", script_attachment)
            .after.call(test_system)
    );
        
    // generate a schedule graph and verify it's what we expect
    let dot_graph = post_update_schedule.render_dot.call();
    
    let expected_dot_graph = `
digraph {
  node_0 [label="bevy_mod_scripting_core::bindings::allocator::garbage_collector"];
  node_1 [label="script_integration_test_harness::dummy_before_post_update_system"];
  node_2 [label="script_integration_test_harness::dummy_post_update_system"];
  node_3 [label="on_test_post_update"];
  node_4 [label="custom_system_a"];
  node_5 [label="custom_system_b"];
  node_6 [label="SystemSet GarbageCollection"];
  node_7 [label="SystemSet ScriptSystem(custom_system_a)"];
  node_8 [label="SystemSet ScriptSystem(custom_system_b)"];
  node_0 -> node_6 [color=red, label="child of", arrowhead=diamond];
  node_4 -> node_7 [color=red, label="child of", arrowhead=diamond];
  node_5 -> node_8 [color=red, label="child of", arrowhead=diamond];
  node_1 -> node_2 [color=blue, label="runs before", arrowhead=normal];
  node_3 -> node_4 [color=blue, label="runs before", arrowhead=normal];
  node_3 -> node_5 [color=blue, label="runs before", arrowhead=normal];
}`; 
    
    assert_str_eq.call(dot_graph, expected_dot_graph, "Expected the schedule graph to match the expected graph");
}