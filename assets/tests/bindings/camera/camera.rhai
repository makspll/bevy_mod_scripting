// a camera without a render target has no viewport
let camera = Camera.new_.call(#{});
assert(type_of(camera.logical_viewport_size.call()) == "()", "logical size is nil without a target");
assert(type_of(camera.logical_viewport_rect.call()) == "()", "logical rect is nil without a target");
assert(type_of(camera.physical_viewport_size.call()) == "()", "physical size is nil without a target");

// projections
let projection_2d = OrthographicProjection.default_2d.call();
assert(projection_2d.scale == 1.0, "2d projection scale");
let projection_3d = OrthographicProjection.default_3d.call();
assert(projection_3d.scale == 1.0, "3d projection scale");
