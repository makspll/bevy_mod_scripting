let res_type = world.get_type_by_name.call("TestResourceWithVariousFields");
let res = world.get_resource.call(res_type);

let map = res.string_map;

let iterator = map.iter();
let count = 0;
let found_keys = #{};

loop {
    let result = iterator.next();
    
    if result == () {
        break;
    }
    
    let key = result[0];
    let value = result[1];
    count += 1;
    found_keys[key] = value;
}

if count != 2 {
    throw `Expected 2 entries, got ${count}`;
}
if found_keys["foo"] != "bar" {
    throw "Expected foo=>bar";
}
if found_keys["zoo"] != "zed" {
    throw "Expected zoo=>zed";
}
