let entity_a = world.spawn_.call();
let componentA = world.get_type_by_name.call("CompWithFromWorldAndComponentData");
let componentB = world.get_type_by_name.call("CompWithDefaultAndComponentData");
let componentC = world.get_type_by_name.call("TestComponent");

world.add_default_component.call(entity_a, componentA);
world.add_default_component.call(entity_a, componentB);
world.insert_component.call(entity_a, componentC, construct.call(componentC, #{
    strings: ["asd"]
}));

let query_result = world.query.call().component.call(componentA).component.call(componentA).component.call(componentC).build.call();

assert(query_result.len == 1, "Expected 1 result, got " + query_result.len);
for (result, i) in query_result {
    assert(result.entity.call().index.call() == entity_a.index.call(), "Expected entity_a, got " + result.entity.call().index.call());
    let components = result.components.call();
    assert(components.len == 3, "Expected 3 components, got " + components.len);
    let A = components[0];
    let B = components[1];
    let C = components[2];
    assert(A[0] == "Default", "Expected 'Default', got: " + A[0]);
    assert(B[0] == "Default", "Expected 'Default', got: " + B[0]);
    assert(C.strings[0] == "asd", "Expected 'asd', got: " + C.strings[0]);
}