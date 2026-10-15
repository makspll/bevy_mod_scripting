-- a camera without a render target has no viewport
local camera = Camera.new({})
assert(camera:logical_viewport_size() == nil, "logical size is nil without a target")
assert(camera:logical_viewport_rect() == nil, "logical rect is nil without a target")
assert(camera:physical_viewport_size() == nil, "physical size is nil without a target")

-- projections
local projection_2d = OrthographicProjection.default_2d()
assert(projection_2d.scale == 1, "2d projection scale")
local projection_3d = OrthographicProjection.default_3d()
assert(projection_3d.scale == 1, "3d projection scale")
