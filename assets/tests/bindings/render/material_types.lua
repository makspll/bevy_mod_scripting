-- the types the fields of materials are made of are in the namespaces of the crates which define them,
-- which are grouped with the bindings which use them
local render = import_namespace("render")
assert(render.AlphaMode ~= nil, "the alpha mode of materials is in the render namespace")
assert(render.OpaqueRendererMethod ~= nil, "the renderer method of materials is in the render namespace")
assert(import_namespace("math").Aabb3d ~= nil, "the bounds of meshes are in the math namespace")
