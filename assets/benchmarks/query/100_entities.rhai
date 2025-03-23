let entity_a = world.spawn_.call();
let entity_b = world.spawn_.call();
let entity_c = world.spawn_.call();

let components = [
    types.CompWithDefaultAndComponentData,
    types.CompWithFromWorldAndComponentData,
    types.SimpleTupleStruct,
    types.SimpleEnum,
];

reseed.call();

for i in 1..=100 {
    let entity = world.spawn_.call();
    let left_to_pick = [0, 1, 2, 3];
    for j in 1..=3 {
        let index = random_int.call(1, left_to_pick.len()) - 1;
        let component = components[left_to_pick[index]];
        left_to_pick.remove(index);
        world.add_default_component.call(entity, component);
    };
};

fn bench() {
    for (result, i) in world.query.call()
        .component.call(types.CompWithFromWorldAndComponentData)
        .component.call(types.SimpleTupleStruct)
        .with_.call(types.SimpleEnum)
        .without.call(types.CompWithDefaultAndComponentData)
        .build.call() {
    };
}