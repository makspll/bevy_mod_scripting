-- meshes from shapes
local Cuboid = import_namespace("math").Cuboid
local Mesh = import_namespace("render").Mesh
local Rectangle = import_namespace("math").Rectangle
local Sphere = import_namespace("math").Sphere
local Vec3 = import_namespace("math").Vec3
local cuboid_mesh = Mesh.from_shape(Cuboid.new(1, 1, 1))
assert(cuboid_mesh:count_vertices() > 0, "cuboid mesh has vertices")
assert(Mesh.from_shape(Sphere.new(1)):count_vertices() > 0, "sphere mesh has vertices")
assert(Mesh.from_shape(Rectangle.new(1, 1)):count_vertices() > 0, "2d shapes make meshes too")

-- values that are not shapes are errors
assert(not pcall(function() return Mesh.from_shape(Vec3.new(1, 2, 3)) end), "non shapes error")
