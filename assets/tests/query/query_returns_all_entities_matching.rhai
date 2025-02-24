let entity_a = world.spawn_.call();
let entity_b = world.spawn_.call();
let entity_c = world.spawn_.call();
let entity_d = world._get_entity_with_test_component.call("CompWithFromWorldAndComponentData");

let component_with = world.get_type_by_name.call("CompWithFromWorldAndComponentData");
let component_without = world.get_type_by_name.call("CompWithDefaultAndComponentData");

world.add_default_component.call(entity_a, component_with);
world.add_default_component.call(entity_b, component_with);
world.add_default_component.call(entity_c, component_with);

world.add_default_component.call(entity_b, component_without);

let found_entities = [];
for (result, i) in world.query.call().component.call(component_with).without.call(component_without).build.call() {
    found_entities.push(result.entity.call());
}

assert(found_entities.len == 3, "Expected 3 entities, got " + found_entities.len);

let expected_entities = [
    entity_d,
    entity_a,
    entity_c,
];

for (entity, i) in found_entities {
    assert(entity.index.call() == expected_entities[i].index.call(), "Expected entity " + expected_entities[i].index.call() + " but got " + entity.index.call());
}

