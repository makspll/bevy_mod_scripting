-- images report their size
local Image = import_namespace("render").Image
local UVec2 = import_namespace("math").UVec2
local Vec2 = import_namespace("math").Vec2
local image = Image.new({})
assert(image:size() == UVec2.new(image:width(), image:height()), "size")
assert(image:size_f32() == Vec2.new(image:width(), image:height()), "size_f32")
