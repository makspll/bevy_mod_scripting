fn on_test() {
    let post_update_schedule = world.get_schedule_by_name.call("PostUpdate");
    
    let script_attachment = ScriptAttachment.new_entity_script.call(entity, script_asset);
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
  node_0 [label="bevy_asset::assets::Assets<bevy_asset::folder::LoadedFolder>::asset_events"];
  node_1 [label="bevy_asset::assets::Assets<bevy_asset::assets::LoadedUntypedAsset>::asset_events"];
  node_2 [label="bevy_asset::assets::Assets<()>::asset_events"];
  node_3 [label="bevy_asset::assets::Assets<bevy_mod_scripting_asset::script_asset::ScriptAsset>::asset_events"];
  node_4 [label="bevy_mod_scripting_bindings::allocator::garbage_collector"];
  node_5 [label="bevy_mod_scripting_core::handler::script_error_logger"];
  node_6 [label="script_integration_test_harness::dummy_before_post_update_system"];
  node_7 [label="script_integration_test_harness::dummy_post_update_system"];
  node_8 [label="on_test_post_update"];
  node_9 [label="custom_system_a"];
  node_10 [label="custom_system_b"];
  node_11 [label="SystemSet AssetEvents"];
  node_12 [label="SystemSet GarbageCollection"];
  node_13 [label="SystemSet ScriptSystem(custom_system_a)"];
  node_14 [label="SystemSet ScriptSystem(custom_system_b)"];
  node_0 -> node_11 [color=red, label="child of", arrowhead=diamond];
  node_1 -> node_11 [color=red, label="child of", arrowhead=diamond];
  node_2 -> node_11 [color=red, label="child of", arrowhead=diamond];
  node_3 -> node_11 [color=red, label="child of", arrowhead=diamond];
  node_4 -> node_12 [color=red, label="child of", arrowhead=diamond];
  node_9 -> node_13 [color=red, label="child of", arrowhead=diamond];
  node_10 -> node_14 [color=red, label="child of", arrowhead=diamond];
  node_6 -> node_7 [color=blue, label="runs before", arrowhead=normal];
  node_8 -> node_9 [color=blue, label="runs before", arrowhead=normal];
  node_8 -> node_10 [color=blue, label="runs before", arrowhead=normal];
}`; 
    
    assert_str_eq.call(dot_graph, expected_dot_graph, "Expected the schedule graph to match the expected graph");
}