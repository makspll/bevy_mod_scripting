-- sprites from a color and a size
local sprite = Sprite.from_color(Color.white(), Vec2.new(10, 20))
assert(sprite.custom_size == Vec2.new(10, 20), "from_color")
assert(Sprite.sized(Vec2.new(3, 4)).custom_size == Vec2.new(3, 4), "sized")
