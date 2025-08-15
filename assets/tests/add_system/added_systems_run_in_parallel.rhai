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
  node_3 [label="bevy_asset::assets::Assets<bevy_mod_scripting_core::asset::ScriptAsset>::asset_events"];
  node_4 [label="bevy_mod_scripting_core::bindings::allocator::garbage_collector"];
  node_5 [label="script_integration_test_harness::dummy_before_post_update_system"];
  node_6 [label="script_integration_test_harness::dummy_post_update_system"];
  node_7 [label="on_test_post_update"];
  node_8 [label="custom_system_a"];
  node_9 [label="custom_system_b"];
  node_10 [label="SystemSet AssetEvents"];
  node_11 [label="SystemSet GarbageCollection"];
  node_12 [label="SystemSet ScriptSystem(custom_system_a)"];
  node_13 [label="SystemSet ScriptSystem(custom_system_b)"];
  node_0 -> node_10 [color=red, label="child of", arrowhead=diamond];
  node_1 -> node_10 [color=red, label="child of", arrowhead=diamond];
  node_2 -> node_10 [color=red, label="child of", arrowhead=diamond];
  node_3 -> node_10 [color=red, label="child of", arrowhead=diamond];
  node_4 -> node_11 [color=red, label="child of", arrowhead=diamond];
  node_8 -> node_12 [color=red, label="child of", arrowhead=diamond];
  node_9 -> node_13 [color=red, label="child of", arrowhead=diamond];
  node_5 -> node_6 [color=blue, label="runs before", arrowhead=normal];
  node_7 -> node_8 [color=blue, label="runs before", arrowhead=normal];
  node_7 -> node_9 [color=blue, label="runs before", arrowhead=normal];
}`; 
    
    assert_str_eq.call(dot_graph, expected_dot_graph, "Expected the schedule graph to match the expected graph");
}