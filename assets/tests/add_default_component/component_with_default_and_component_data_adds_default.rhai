let entity = world.spawn_.call();
let _type = world.get_type_by_name.call("CompWithDefaultAndComponentData");
world.add_default_component.call(entity, _type);

let added = world.has_component.call(entity, _type);
assert(type_of(added) != "()", "Component not added");

let component = world.get_component.call(entity, _type);
assert(component[0] == "Default", "Component did not have default value, got: " + component[0]);