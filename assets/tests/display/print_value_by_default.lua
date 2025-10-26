local vec = Vec3.new(1, 2, 3)

assert(vec:display() == "Vec3 { x: 1.0, y: 2.0, z: 3.0 }",
    "Vec3 display incorrect, expected Vec3 { x: 1.0, y: 2.0, z: 3.0 } but got " .. vec:display())


-- multiline
local function strip_alloc_ids(s)
    s = s:gsub("ReflectAllocationId%b()", "ReflectAllocationId()")
    return s
end

local expected_vec3_debug = [[
 ReflectReference {
    base: ReflectBaseType {
        type_id: TypeId(
            "glam::Vec3",
        ),
        base_id: Owned(
            ReflectAllocationId(*anything*),
        ),
    },
    reflect_path: ReferencePath {
        one_indexed: false,
        path: [],
    },
}
]]
-- normalize allocation ids before comparison so tests don't fail on runtime-generated ids
do
    local actual = vec:debug()
    local expected = expected_vec3_debug
    actual = strip_alloc_ids(actual)
    expected = strip_alloc_ids(expected)
    assert_str_eq(actual, expected)
end


local test_resource = world.get_resource(types.TestResource)

local expected_test_resource_debug = [[
ReflectReference {
    base: ReflectBaseType {
        type_id: TypeId(
            "test_utils::test_data::TestResource",
        ),
        base_id: Resource(
            ComponentId(
                test_utils::test_data::TestResource,
            ),
        ),
    },
    reflect_path: ReferencePath {
        one_indexed: false,
        path: [],
    },
}
]]

-- normalize allocation ids before comparison so tests don't fail on runtime-generated ids
do
    local actual = test_resource:debug()
    local expected = expected_test_resource_debug
    actual = strip_alloc_ids(actual)
    expected = strip_alloc_ids(expected)
    assert_str_eq(actual, expected,
        "TestResource debug incorrect, expected " .. expected .. " but got " .. actual)
end


assert_str_eq(test_resource:display(), "TestResource { bytes: [0, 1, 2, 3, 4, 5] }",
    "TestResource display incorrect, expected TestResource { bytes: [0, 1, 2, 3, 4, 5] } but got " ..
    test_resource:display())
