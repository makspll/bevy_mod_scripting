// the test app has no windows

assert(Window.primary.call() == (), "primary is nil without windows");
assert(Window.primary_entity.call() == (), "primary_entity is nil without windows");

// resolutions
let resolution = WindowResolution.new_.call(800, 600);
assert(resolution.physical_width == 800, "physical_width");
assert(resolution.physical_height == 600, "physical_height");
assert(resolution.width.call() == 800.0, "width");
assert(resolution.height.call() == 600.0, "height");
resolution.set.call(1024.0, 768.0);
assert(resolution.width.call() == 1024.0, "set width");
assert(resolution.height.call() == 768.0, "set height");
