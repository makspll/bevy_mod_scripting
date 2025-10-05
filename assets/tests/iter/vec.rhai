let res_type = world.get_type_by_name.call("TestResourceWithVariousFields");
let res = world.get_resource.call(res_type);

let iterated_vals = [];
let iterator = res.vec_usize.iter();

loop {
    let result = iterator.next();
    
    if result == () {
        break;
    }
    
    iterated_vals.push(result);
}

assert(iterated_vals.len == 5, "Length is not 5");
assert(iterated_vals[0] == 1, "First value is not 1");
assert(iterated_vals[1] == 2, "Second value is not 2");
assert(iterated_vals[2] == 3, "Third value is not 3");
assert(iterated_vals[3] == 4, "Fourth value is not 4");
assert(iterated_vals[4] == 5, "Fifth value is not 5");
