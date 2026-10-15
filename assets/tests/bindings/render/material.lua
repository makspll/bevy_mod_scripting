-- materials from colors
local material = StandardMaterial.from_color(Color.srgb(1, 0, 0))
assert(material.base_color == Color.srgb(1, 0, 0), "base color is the given color")
assert(StandardMaterial.from_color(Color.srgb(0, 1, 0)).base_color ~= Color.srgb(1, 0, 0), "other colors differ")
