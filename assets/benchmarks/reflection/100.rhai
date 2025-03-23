let Component = world.register_new_component.call("DeepComponent");
let new_entity = world.spawn_.call();

reseed.call();

let keys = [
    "_0",
    "my_key",
    "longish_key"
];

fn make_path(step_keys, index) {
    if index >= step_keys.len() {
        return #{};
    };
    let key = step_keys[index];
    let nested = make_path(step_keys, index + 1);
    let obj = #{};
    obj.set(key,nested);
    return obj;
};

let current_reference = ();
let steps = 100;
let step_keys = [];

fn pre_bench() {
    // Choose keys for every step.
    for i in 0 .. steps {
        let key = keys[random_int.call(0, keys.len() - 1)];
        step_keys.push(key);
    };

    // Build the nested table.
    let instance = make_path(step_keys, 0);
    world.remove_component.call(new_entity, Component);
    world.insert_component.call(new_entity, Component, construct.call(types.DynamicComponent, #{ "data": instance }));
    current_reference = world.get_component.call(new_entity, Component);
};

fn bench() {
    // reference into random key into current_reference steps times
    let reference = current_reference.data;
    let current_step = 0;
    while current_step < steps {
        reference = reference[step_keys[current_step]];
        current_step += 1;
    };
};