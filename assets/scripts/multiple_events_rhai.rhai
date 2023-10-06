fn on_init(name) {
    print(`Hello World! From "${name}" in Init`);
}

fn on_update(name, delta) {
    // print hello world from {name}: {time}
    print(`Hello World! From "${name}" in Update: ${delta}`);
}