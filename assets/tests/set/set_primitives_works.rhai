let Resource = world.get_type_by_name.call("TestResourceWithVariousFields");
let resource = world.get_resource.call(Resource);

resource.string = "Hello, World!";
resource.bool = true;
resource.int = 42;
resource.float = 3.0;
resource.vec_usize = [ 1, 2 ];

assert(resource.string == "Hello, World!", "Expected 'Hello, World!', got " + resource.string);
assert(resource.bool == true, "Expected true, got " + resource.bool);
assert(resource.int == 42, "Expected 42, got " + resource.int);
assert(resource.float == 3.0, "Expected 3.14, got " + resource.float);
assert(resource.vec_usize[0] == 1, "Expected 1, got " + resource.vec_usize[1]);

resource.string = "Goodbye, World!";
resource.bool = false;
resource.int = 24;
resource.float = 1.0;
resource.vec_usize = [ 3, 4 ];

assert(resource.string == "Goodbye, World!", "Expected 'Goodbye, World!', got " + resource.string);
assert(resource.bool == false, "Expected false, got " + resource.bool);
assert(resource.int == 24, "Expected 24, got " + resource.int);
assert(resource.float == 1.0, "Expected 1.41, got " + resource.float);
assert(resource.vec_usize[0] == 3, "Expected 3, got " + resource.vec_usize[1]);


