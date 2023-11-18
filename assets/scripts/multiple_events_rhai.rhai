fn on_init(name) {
    print(`Hello World! From "${name}" in Init`);

    let parent = world.get_parent(entity);
    if parent == () {
        print(`Parent doesn't exist`);
    } else {
        print(`Parent exists`);
    }
}

fn on_update(name, delta) {
    print(`Hello World! From "${name}" in Update: ${delta}`);

    world.despawn_recursive(entity);
}