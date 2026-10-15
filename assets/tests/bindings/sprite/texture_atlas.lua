-- a grid of 4 columns and 2 rows has 8 tiles
local layout = TextureAtlasLayout.from_grid(UVec2.new(16, 16), 4, 2, nil, nil)
assert(layout:len() == 8, "from_grid")
