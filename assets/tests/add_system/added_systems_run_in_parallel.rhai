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
  node_3 [label="bevy_asset::assets::Assets<test_utils::test_data::TestAsset>::asset_events"];
  node_4 [label="bevy_asset::assets::Assets<bevy_mod_scripting_asset::script_asset::ScriptAsset>::asset_events"];
  node_5 [label="bevy_mod_scripting_bindings::allocator::garbage_collector"];
  node_6 [label="bevy_mod_scripting_core::handler::script_error_logger"];
  node_7 [label="script_integration_test_harness::dummy_before_post_update_system"];
  node_8 [label="script_integration_test_harness::dummy_post_update_system"];
  node_9 [label="bevy_mod_scripting_core::pipeline::start::filter_script_attachments<bevy_mod_scripting_rhai::RhaiScriptingPlugin>"];
  node_10 [label="bevy_mod_scripting_core::pipeline::start::filter_script_detachments<bevy_mod_scripting_rhai::RhaiScriptingPlugin>"];
  node_11 [label="bevy_mod_scripting_core::pipeline::start::filter_script_modifications<bevy_mod_scripting_rhai::RhaiScriptingPlugin>"];
  node_12 [label="bevy_mod_scripting_core::pipeline::start::process_attachments<bevy_mod_scripting_rhai::RhaiScriptingPlugin>"];
  node_13 [label="bevy_mod_scripting_core::pipeline::start::process_detachments<bevy_mod_scripting_rhai::RhaiScriptingPlugin>"];
  node_14 [label="bevy_mod_scripting_core::pipeline::start::process_asset_modifications<bevy_mod_scripting_rhai::RhaiScriptingPlugin>"];
  node_15 [label="bevy_mod_scripting_core::pipeline::automatic_pipeline_runner<bevy_mod_scripting_rhai::RhaiScriptingPlugin>"];
  node_16 [label="on_test_post_update"];
  node_17 [label="custom_system_a"];
  node_18 [label="custom_system_b"];
  node_19 [label="SystemSet AssetEvents"];
  node_20 [label="SystemSet GarbageCollection"];
  node_21 [label="SystemSet ListeningPhase"];
  node_22 [label="SystemSet MachineStartPhase"];
  node_23 [label="SystemSet ScriptSystem(custom_system_a)"];
  node_24 [label="SystemSet ScriptSystem(custom_system_b)"];
  node_0 -> node_19 [color=red, label="child of", arrowhead=diamond];
  node_1 -> node_19 [color=red, label="child of", arrowhead=diamond];
  node_2 -> node_19 [color=red, label="child of", arrowhead=diamond];
  node_3 -> node_19 [color=red, label="child of", arrowhead=diamond];
  node_4 -> node_19 [color=red, label="child of", arrowhead=diamond];
  node_5 -> node_20 [color=red, label="child of", arrowhead=diamond];
  node_9 -> node_21 [color=red, label="child of", arrowhead=diamond];
  node_10 -> node_21 [color=red, label="child of", arrowhead=diamond];
  node_11 -> node_21 [color=red, label="child of", arrowhead=diamond];
  node_12 -> node_22 [color=red, label="child of", arrowhead=diamond];
  node_13 -> node_22 [color=red, label="child of", arrowhead=diamond];
  node_14 -> node_22 [color=red, label="child of", arrowhead=diamond];
  node_17 -> node_23 [color=red, label="child of", arrowhead=diamond];
  node_18 -> node_24 [color=red, label="child of", arrowhead=diamond];
  node_7 -> node_8 [color=blue, label="runs before", arrowhead=normal];
  node_9 -> node_10 [color=blue, label="runs before", arrowhead=normal];
  node_9 -> node_10 [color=blue, label="runs before", arrowhead=normal];
  node_9 -> node_11 [color=blue, label="runs before", arrowhead=normal];
  node_10 -> node_11 [color=blue, label="runs before", arrowhead=normal];
  node_12 -> node_13 [color=blue, label="runs before", arrowhead=normal];
  node_13 -> node_14 [color=blue, label="runs before", arrowhead=normal];
  node_16 -> node_17 [color=blue, label="runs before", arrowhead=normal];
  node_16 -> node_18 [color=blue, label="runs before", arrowhead=normal];
  node_21 -> node_22 [color=blue, label="runs before", arrowhead=normal];
  node_22 -> node_15 [color=blue, label="runs before", arrowhead=normal];
}`; 
    
    assert_str_eq.call(dot_graph, expected_dot_graph, "Expected the schedule graph to match the expected graph");
}