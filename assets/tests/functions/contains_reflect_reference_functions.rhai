
let Resource = world.get_type_by_name.call("TestResource");
let resource = world.get_resource.call(Resource);

let functions = resource.functions.call();
assert(functions.len() > 0, "functions should not be empty");

let available_names = [];

for function_ref in functions {
    available_names.push(function_ref.name);
}

assert("display" in available_names, "functions should contain display, but got: " + available_names);