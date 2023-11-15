fn on_init(name) {
    print(`Hello World! From "${name}" in Init`);

    let parent = world.get_parent(entity);
    print(`Parent: ${type_of(parent)}`);
}

fn on_update(name, delta) {
    print(`Hello World! From "${name}" in Update: ${delta}`);

    world.despawn_recursive(entity);
}