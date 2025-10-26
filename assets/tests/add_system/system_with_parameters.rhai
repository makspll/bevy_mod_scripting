let runs = [];

let ResourceTypeA = world.get_type_by_name.call("TestResource");
let ResourceTypeB = world.get_type_by_name.call("TestResourceWithVariousFields");
let ComponentA   = world.get_type_by_name.call("CompWithFromWorldAndComponentData");
let ComponentB   = world.get_type_by_name.call("CompWithDefaultAndComponentData");

fn on_test() {
    let post_update_schedule = world.get_schedule_by_name.call("PostUpdate");
    
    let script_attachment = ScriptAttachment.new_entity_script.call(entity, script_asset);
    let entity  = world.spawn_.call();
    let entity2 = world.spawn_.call();
    
    world.add_default_component.call(entity, ComponentA);
    world.add_default_component.call(entity, ComponentB);
    world.add_default_component.call(entity2, ComponentA);
    world.add_default_component.call(entity2, ComponentB);


    let built_system = system_builder.call("my_parameterised_system", script_attachment)
                            .resource.call(ResourceTypeA)
                            .query.call(world.query.call().component.call(ComponentA).with_.call(ComponentB))
                            .resource.call(ResourceTypeB);

    world.add_system.call(post_update_schedule, built_system);

    return true;
}

fn my_parameterised_system(resourceA, query, resourceB) {
    print("my_parameterised_system");
    runs.push("my_non_exclusive_system");

    assert(type_of(resourceA) != (), "Expected to get resource but got nil");
    assert(type_of(query) != (), "Expected to get query but got nil");
    assert(type_of(resourceB) != (), "Expected to get resource but got nil");

    assert(resourceA.bytes.len() == 6, "Expected 6 bytes, got: " + resourceA.bytes.len());
    assert(resourceB.string == "Initial Value", "Expected 'Initial Value', got: " + resourceB.string);
    assert(query.len() == 2, "Expected 3 results, got: " + query.len());
    for result in query {
        components = result.components.call();
        let componentA = components[0];
        let componentB = components[1];
        assert(componentA["_0"] == "Default", "Expected 'Default', got: " + componentA["_0"]);
        assert(componentB["_0"] == "Default", "Expected 'Default', got: " + componentB["_0"]);
    }
}

fn on_test_post_update() {
}

fn on_test_last() {
    assert(runs.len == 1, "Expected 1 runs, got: " + runs.len);
}