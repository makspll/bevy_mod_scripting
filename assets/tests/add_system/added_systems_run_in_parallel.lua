function on_test()
  local post_update_schedule = world.get_schedule_by_name("PostUpdate")

  local test_system = post_update_schedule:get_system_by_name("on_test_post_update")

  local script_attachment = ScriptAttachment.new_entity_script(entity, script_asset)

  local system_a = world.add_system(
    post_update_schedule,
    system_builder("custom_system_a", script_attachment)
    :after(test_system)
  )

  local system_b = world.add_system(
    post_update_schedule,
    system_builder("custom_system_b", script_attachment)
    :after(test_system)
  )

  -- generate a schedule graph and verify it's what we expect
  local dot_graph = post_update_schedule:render_dot()

  local expected_dot_graph = [[
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
  node_9 [label="on_test_post_update"];
  node_10 [label="custom_system_a"];
  node_11 [label="custom_system_b"];
  node_12 [label="SystemSet AssetEventSystems"];
  node_13 [label="SystemSet GarbageCollection"];
  node_14 [label="SystemSet ScriptSystem(custom_system_a)"];
  node_15 [label="SystemSet ScriptSystem(custom_system_b)"];
  node_0 -> node_12 [color=red, label="child of", arrowhead=diamond];
  node_1 -> node_12 [color=red, label="child of", arrowhead=diamond];
  node_2 -> node_12 [color=red, label="child of", arrowhead=diamond];
  node_3 -> node_12 [color=red, label="child of", arrowhead=diamond];
  node_4 -> node_12 [color=red, label="child of", arrowhead=diamond];
  node_5 -> node_13 [color=red, label="child of", arrowhead=diamond];
  node_10 -> node_14 [color=red, label="child of", arrowhead=diamond];
  node_11 -> node_15 [color=red, label="child of", arrowhead=diamond];
  node_7 -> node_8 [color=blue, label="runs before", arrowhead=normal];
  node_9 -> node_10 [color=blue, label="runs before", arrowhead=normal];
  node_9 -> node_11 [color=blue, label="runs before", arrowhead=normal];
}
    ]]
  assert_str_eq(dot_graph, expected_dot_graph, "Expected the schedule graph to match the expected graph")
end
