local vec = Vec3.new(1, 2, 3)

assert(vec:display() == "Vec3 { x: 1.0, y: 2.0, z: 3.0 }",
    "Vec3 display incorrect, expected Vec3 { x: 1.0, y: 2.0, z: 3.0 } but got " .. vec:display())


-- multiline
local expected_vec3_debug = [[
 ReflectReference {
    base: ReflectBaseType {
        type_id: TypeId(
            "glam::Vec3",
        ),
        base_id: Owned(
            ReflectAllocationId(
                385,
            ),
        ),
    },
    reflect_path: ParsedPath(
        [],
    ),
}
]]
assert_str_eq(vec:debug(), expected_vec3_debug);


local test_resource = world.get_resource(types.TestResource)

local expected_test_resource_debug = [[
ReflectReference {
    base: ReflectBaseType {
        type_id: TypeId(
            "test_utils::test_data::TestResource",
        ),
        base_id: Resource(
            ComponentId(
                "test_utils::test_data::TestResource",
            ),
        ),
    },
    reflect_path: ParsedPath(
        [],
    ),
}
]]

assert_str_eq(test_resource:debug(), expected_test_resource_debug,
    "TestResource debug incorrect, expected " .. expected_test_resource_debug .. " but got " .. test_resource:debug())


assert_str_eq(test_resource:display(), "TestResource { bytes: [0, 1, 2, 3, 4, 5] }",
    "TestResource display incorrect, expected TestResource { bytes: [0, 1, 2, 3, 4, 5] } but got " ..
    test_resource:display())
