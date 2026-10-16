let layers = RenderLayers.layer.call(2);
assert(layers.intersects.call(RenderLayers.layer.call(2)), "same layer intersects");
assert(!layers.intersects.call(RenderLayers.layer.call(3)), "other layer does not intersect");
let none = RenderLayers.none.call();
assert(!none.intersects.call(RenderLayers.none.call()), "no layers intersect nothing");

let both = layers.with_.call(5);
assert(both.intersects.call(RenderLayers.layer.call(5)), "with adds a layer");
let without_two = both.without.call(2);
assert(!without_two.intersects.call(RenderLayers.layer.call(2)), "without removes a layer");
assert(without_two.intersects.call(RenderLayers.layer.call(5)), "without keeps other layers");

let listed = both.iter.call();
assert(listed.len() == 2 && listed[0] == 2 && listed[1] == 5, "iter lists the layers in order");
let empty = none.iter.call();
assert(empty.len() == 0, "no layers iter to nothing");

let far = RenderLayers.layer.call(100);
let far_none = RenderLayers.none.call();
let far_layers = far_none.with_.call(100);
assert(far.intersects.call(far_layers), "layers past one block");
