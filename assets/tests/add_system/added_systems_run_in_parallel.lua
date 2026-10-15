local ScriptAttachment = import_namespace("bms").ScriptAttachment
function on_test()
  local post_update_schedule = world.get_schedule_by_name("PostUpdate")

  local test_system = post_update_schedule:get_system_by_name("on_test_post_update")

  local script_attachment = ScriptAttachment.new_entity_script(entity, script_asset)

  local system_a = world.add_system(
    post_update_schedule,
    system_builder("custom_system_a", script_attachment)
    :after(test_system)
  )

  assert_str_eq(system_a:identifier(), "custom_system_a")

  local system_b = world.add_system(
    post_update_schedule,
    system_builder("custom_system_b", script_attachment)
    :after(test_system)
  )

  assert_str_eq(system_b:identifier(), "custom_system_b")

  -- the new systems both run after the existing one, but run in parallel (no ordering between them)
  world._assert_system_runs_before(post_update_schedule, "on_test_post_update", "custom_system_a")
  world._assert_system_runs_before(post_update_schedule, "on_test_post_update", "custom_system_b")
  world._assert_systems_run_in_parallel(post_update_schedule, "custom_system_a", "custom_system_b")
end
