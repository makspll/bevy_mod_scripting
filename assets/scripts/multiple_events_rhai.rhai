fn on_init(name) {
    print(`Hello World! From "${name}" in Init`);
}

fn on_update(name, delta) {
    print(`Hello World! From "${name}" in Update: ${delta}`);

    world.despawn_recursive(entity);
}