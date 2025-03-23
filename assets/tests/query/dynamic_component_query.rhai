let entity_a = world.spawn_.call();
let NewComponent = world.register_new_component.call("NewComponent");

world.add_default_component.call(entity_a, NewComponent);

let found_entities = [];
for (result, i) in world.query.call().component.call(NewComponent).build.call() {
    found_entities.push(result.entity.call());
}

assert(found_entities.len == 1, "Expected 1 entities, got " + found_entities.len);

let expected_entities = [entity_a];

for (entity, i) in found_entities {
    assert(entity.index.call() == expected_entities[i].index.call(), "Expected entity " + expected_entities[i].index.call() + " but got " + entity.index.call());
}