// anchors are offsets of a sprite's size from its center
assert(Anchor.center.call().as_vec.call() == Vec2.new_.call(0.0, 0.0), "center");
assert(Anchor.center_left.call().as_vec.call() == Vec2.new_.call(-0.5, 0.0), "center_left");
assert(Anchor.center_right.call().as_vec.call() == Vec2.new_.call(0.5, 0.0), "center_right");
assert(Anchor.bottom_left.call().as_vec.call() == Vec2.new_.call(-0.5, -0.5), "bottom_left");
assert(Anchor.bottom_center.call().as_vec.call() == Vec2.new_.call(0.0, -0.5), "bottom_center");
assert(Anchor.bottom_right.call().as_vec.call() == Vec2.new_.call(0.5, -0.5), "bottom_right");
assert(Anchor.top_left.call().as_vec.call() == Vec2.new_.call(-0.5, 0.5), "top_left");
assert(Anchor.top_center.call().as_vec.call() == Vec2.new_.call(0.0, 0.5), "top_center");
assert(Anchor.top_right.call().as_vec.call() == Vec2.new_.call(0.5, 0.5), "top_right");
