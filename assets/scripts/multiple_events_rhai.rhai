fn on_init(name) {
    print(`Hello World! From "${name}" in Init`);

    let parent = world.get_parent(entity);
    switch type_of(parent) {
        "bevy_ecs::entity::Entity" => {
            print(`Parent is an Entity`);
        },
        "()" => {
            print(`Parent doesn't exist`);
        },
    }
}

fn on_update(name, delta) {
    print(`Hello World! From "${name}" in Update: ${delta}`);

    world.despawn_recursive(entity);
}