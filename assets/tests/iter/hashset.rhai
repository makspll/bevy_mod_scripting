let res_type = world.get_type_by_name.call("TestResourceWithVariousFields");
let res = world.get_resource.call(res_type);

let set = res.string_set;

let iterator = set.iter();
let count = 0;
let found_values = #{};

loop {
    let result = iterator.next();
    
    if result == () {
        break;
    }
    
    count += 1;
    found_values[result] = true;
}

if count != 2 {
    throw `Expected 2 entries, got ${count}`;
}
if found_values["apple"] != true {
    throw "Expected to find 'apple'";
}
if found_values["banana"] != true {
    throw "Expected to find 'banana'";
}
