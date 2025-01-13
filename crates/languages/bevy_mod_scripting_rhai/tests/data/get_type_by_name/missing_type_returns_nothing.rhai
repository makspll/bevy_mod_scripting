let my_fn = world["get_type_by_name"];
print(my_fn);
let type = my_fn("UnregisteredType");
assert(type == (), "Unregistered type was found");
