let world = import_namespace.call("bms").world;
let test_utils = import_namespace.call("test_utils");
let entity_a = world.spawn_.call();
let entity_b = world.spawn_.call();
let entity_c = world.spawn_.call();

let components = [
    world.get_type_by_name.call("test_utils::test_data::CompWithDefaultAndComponentData"),
    world.get_type_by_name.call("test_utils::test_data::CompWithFromWorldAndComponentData"),
    world.get_type_by_name.call("test_utils::test_data::SimpleTupleStruct"),
    world.get_type_by_name.call("test_utils::test_data::SimpleEnum"),
];

test_utils.reseed.call();

for i in 1..=1000 {
    let entity = world.spawn_.call();
    // spawn 1000 entities with random components
    let left_to_pick = [0, 1, 2, 3];
    for j in 1..=3 {
        let index = test_utils.random_int.call(1, left_to_pick.len()) - 1;
        let component = components[left_to_pick[index]];
        left_to_pick.remove(index);
        world.add_default_component.call(entity, component);
    };
};

fn bench() {
    for (result, i) in world.query.call()
        .component.call(world.get_type_by_name.call("test_utils::test_data::CompWithFromWorldAndComponentData"))
        .component.call(world.get_type_by_name.call("test_utils::test_data::SimpleTupleStruct"))
        .with_.call(world.get_type_by_name.call("test_utils::test_data::SimpleEnum"))
        .without.call(world.get_type_by_name.call("test_utils::test_data::CompWithDefaultAndComponentData"))
        .build.call() {
    };
}