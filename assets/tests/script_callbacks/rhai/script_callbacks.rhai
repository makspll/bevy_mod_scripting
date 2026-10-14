// note: `world`, `entity`, and `script_asset` are only visible inside callbacks, not in helper functions

fn double(x) {
    return x * 2;
}

fn add(a, b) {
    return a + b;
}

fn bounce(back_to) {
    return run_script_callback.call("double", ScriptAttachment.new_entity_script.call(back_to, script_asset), 1);
}

fn record(value) {
    world.insert_component.call(entity, world.get_type_by_name.call("TestComponent"), TestComponent.new_.call(#{ strings: [value] }));
}

fn on_test() {
    // the other entity running this script
    let other = ();
    let found = false;
    for (result, i) in world.query.call().component.call(world.get_type_by_name.call("ScriptComponent")).build.call() {
        let e = result.entity.call();
        if e.index.call().index.call() != entity.index.call().index.call() {
            other = e;
            found = true;
        }
    }
    assert(found, "could not find the other entity");
    let other_attachment = ScriptAttachment.new_entity_script.call(other, script_asset);
    let own_attachment = ScriptAttachment.new_entity_script.call(entity, script_asset);

    // immediate callbacks return what the callback returned
    let doubled = run_script_callback.call("double", other_attachment, 21);
    assert(doubled == 42, "Expected 42, got: " + doubled);
    let sum = run_script_callback.call("add", other_attachment, 1, 2);
    assert(sum == 3, "Expected 3, got: " + sum);

    // recipients return a list with one value per script
    let results = run_script_callback.call("double", Recipients.entity_script.call(other, script_asset), 5);
    assert(results.len == 1, "Expected 1 result, got: " + results.len);
    assert(results[0] == 10, "Expected 10, got: " + results[0]);

    // busy contexts error instead of deadlocking
    assert_throws(|| {
        run_script_callback.call("double", own_attachment, 1);
    }, "it is the script currently running");
    assert_throws(|| {
        run_script_callback.call("bounce", other_attachment, entity);
    }, "further up the current chain");

    // queued callbacks can target this script, and only run once this callback finishes
    queue_script_callback.call("record", own_attachment, "queued!");
    // flushing the world must not run them early
    world.spawn_.call();
    assert(!world.has_component.call(entity, world.get_type_by_name.call("TestComponent")), "Queued callback ran before the callback queueing it finished");
    return "ok";
}

fn on_test_post_update() {
    return world.get_component.call(entity, world.get_type_by_name.call("TestComponent")).strings[0];
}
