// Create a Vec3 and verify its display/debug output matches the Lua test expectations
let vec = Vec3.new_.call(1.0, 2.0, 3.0);

let display_str = vec.to_string();
assert(display_str == "Vec3 { x: 1.0, y: 2.0, z: 3.0 }",
	"Vec3 display incorrect, expected Vec3 { x: 1.0, y: 2.0, z: 3.0 } but got " + display_str);

let expected_vec3_debug = `
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
`;

assert_str_eq.call(vec.to_debug(), expected_vec3_debug);
