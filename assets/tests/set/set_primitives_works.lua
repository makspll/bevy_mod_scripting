local Resource = world.get_type_by_name("TestResourceWithVariousFields")
local resource = world.get_resource(Resource)

resource.string = "Hello, World!"
resource.bool = true
resource.int = 42
resource.float = 3.0
resource.vec_usize = { 1, 2 }
resource.string_map = { foo = "hello", zoo = "world" }

assert(resource.string == "Hello, World!", "Expected 'Hello, World!', got " .. resource.string)
assert(resource.bool == true, "Expected true, got " .. tostring(resource.bool))
assert(resource.int == 42, "Expected 42, got " .. resource.int)
assert(resource.float == 3.0, "Expected 3.14, got " .. resource.float)
assert(resource.vec_usize[1] == 1, "Expected 1, got " .. resource.vec_usize[1])
assert(resource.string_map:len() == 2, "Expected 2, got " .. resource.string_map:len())
-- assert(resource.string_map["foo"] == "hello", "Expected 'hello', got " .. resource.string_map["foo"])
-- assert(resource.string_map["zoo"] == "world", "Expected 'world', got " .. resource.string_map["zoo"])

resource.string = "Goodbye, World!"
resource.bool = false
resource.int = 24
resource.float = 1.0
resource.vec_usize = { 3, 4 }
resource.string_map = { foo = "goodbye", zoo = "world" }

assert(resource.string == "Goodbye, World!", "Expected 'Goodbye, World!', got " .. resource.string)
assert(resource.bool == false, "Expected false, got " .. tostring(resource.bool))
assert(resource.int == 24, "Expected 24, got " .. resource.int)
assert(resource.float == 1.0, "Expected 1.41, got " .. resource.float)
assert(resource.string_map:len() == 2, "Expected 2, got " .. resource.string_map:len())
-- assert(resource.string_map["foo"] == "goodbye", "Expected 'goodbye', got " .. resource.string_map["foo"])
-- assert(resource.string_map["zoo"] == "world", "Expected 'world', got " .. resource.string_map["zoo"])

