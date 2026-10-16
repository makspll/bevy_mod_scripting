// meshes from shapes
let Cuboid = import_namespace.call("math").Cuboid;
let Mesh = import_namespace.call("render").Mesh;
let Rectangle = import_namespace.call("math").Rectangle;
let cuboid_mesh = Mesh.from_shape.call(Cuboid.new_.call(1.0, 1.0, 1.0));
assert(cuboid_mesh.count_vertices.call() > 0, "cuboid mesh has vertices");
assert(Mesh.from_shape.call(Rectangle.new_.call(1.0, 1.0)).count_vertices.call() > 0, "2d shapes make meshes too");
