// sprites from a color and a size
let Color = import_namespace.call("color").Color;
let Sprite = import_namespace.call("sprite").Sprite;
let Vec2 = import_namespace.call("math").Vec2;
let sprite = Sprite.from_color.call(Color.white.call(), Vec2.new_.call(10.0, 20.0));
assert(sprite.custom_size == Vec2.new_.call(10.0, 20.0), "from_color");
assert(Sprite.sized.call(Vec2.new_.call(3.0, 4.0)).custom_size == Vec2.new_.call(3.0, 4.0), "sized");
