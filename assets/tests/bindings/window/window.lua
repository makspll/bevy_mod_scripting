-- the test app has no windows
local Window = import_namespace("window").Window
local WindowResolution = import_namespace("window").WindowResolution

assert(Window.primary() == nil, "primary is nil without windows")
assert(Window.primary_entity() == nil, "primary_entity is nil without windows")

-- resolutions
local resolution = WindowResolution.new(800, 600)
assert(resolution.physical_width == 800, "physical_width")
assert(resolution.physical_height == 600, "physical_height")
assert(resolution:width() == 800, "width")
assert(resolution:height() == 600, "height")
resolution:set(1024.0, 768.0)
assert(resolution:width() == 1024.0, "set width")
assert(resolution:height() == 768.0, "set height")
