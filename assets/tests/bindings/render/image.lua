-- images report their size
local image = Image.new({})
assert(image:size() == UVec2.new(image:width(), image:height()), "size")
assert(image:size_f32() == Vec2.new(image:width(), image:height()), "size_f32")
