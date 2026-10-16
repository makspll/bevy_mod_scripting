let world = import_namespace.call("bms").world;
let type = world.get_type_by_name.call("MissingType");

assert(type == (), "Unregistered type was found");
