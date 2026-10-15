// materials from colors
let material = StandardMaterial.from_color.call(Color.srgb.call(1.0, 0.0, 0.0));
assert(material.base_color == Color.srgb.call(1.0, 0.0, 0.0), "base color is the given color");
assert(StandardMaterial.from_color.call(Color.srgb.call(0.0, 1.0, 0.0)).base_color != Color.srgb.call(1.0, 0.0, 0.0), "other colors differ");
