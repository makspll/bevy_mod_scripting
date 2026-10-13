let first = world.register_new_component.call("ScriptComponentAllowExisting");
let second = world.register_new_component.call("ScriptComponentAllowExisting", true);

let entity = world.spawn_.call();
world.add_default_component.call(entity, first);

assert(world.has_component.call(entity, second), "allow_existing should return the original component");
