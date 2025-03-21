let NewComponent = world.register_new_component.call("ScriptComponentA");
let new_entity = world.spawn_.call();
world.add_default_component.call(new_entity, NewComponent);

let component_instance = world.get_component.call(new_entity, NewComponent);
assert(type_of(component_instance) != "()", "unexpected value: " + component_instance.data);

world.remove_component.call(new_entity, NewComponent);
let component_instance_after = world.get_component.call(new_entity, NewComponent);

assert(type_of(component_instance_after) == "()", "unexpected value: " + component_instance_after);