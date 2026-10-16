let world = import_namespace.call("bms").world;
let NewComponent = world.register_new_component.call("script.test", "ScriptComponentA");
let entity = world.spawn_.call();

assert(world.has_component.call(entity, NewComponent) == false, "Entity should not have component");
world.add_default_component.call(entity, NewComponent);
assert(world.has_component.call(entity, NewComponent) == true, "Entity should have component");