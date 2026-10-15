-- sprites from a color and a size
local Color = import_namespace("color").Color
local Sprite = import_namespace("sprite").Sprite
local Vec2 = import_namespace("math").Vec2
local sprite = Sprite.from_color(Color.white(), Vec2.new(10, 20))
assert(sprite.custom_size == Vec2.new(10, 20), "from_color")
assert(Sprite.sized(Vec2.new(3, 4)).custom_size == Vec2.new(3, 4), "sized")
