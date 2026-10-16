// a grid of 4 columns and 2 rows has 8 tiles
let TextureAtlasLayout = import_namespace.call("sprite").TextureAtlasLayout;
let UVec2 = import_namespace.call("math").UVec2;
let layout = TextureAtlasLayout.from_grid.call(UVec2.new_.call(16, 16), 4, 2, (), ());
assert(layout.len.call() == 8, "from_grid");
