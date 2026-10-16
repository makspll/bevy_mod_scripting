// a grid of 4 columns and 2 rows has 8 tiles
let layout = TextureAtlasLayout.from_grid.call(UVec2.new_.call(16, 16), 4, 2, (), ());
assert(layout.len.call() == 8, "from_grid");
