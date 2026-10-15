// the types the fields of materials are made of are in the namespaces of the crates which define them,
// which are grouped with the bindings which use them
let render = import_namespace.call("render");
assert(type_of(render.AlphaMode) != "()", "the alpha mode of materials is in the render namespace");
assert(type_of(render.OpaqueRendererMethod) != "()", "the renderer method of materials is in the render namespace");
assert(type_of(import_namespace.call("math").Aabb3d) != "()", "the bounds of meshes are in the math namespace");
