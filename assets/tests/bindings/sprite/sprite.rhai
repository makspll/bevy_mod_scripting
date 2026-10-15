// sprites from a color and a size
let sprite = Sprite.from_color.call(Color.white.call(), Vec2.new_.call(10.0, 20.0));
assert(sprite.custom_size == Vec2.new_.call(10.0, 20.0), "from_color");
assert(Sprite.sized.call(Vec2.new_.call(3.0, 4.0)).custom_size == Vec2.new_.call(3.0, 4.0), "sized");
