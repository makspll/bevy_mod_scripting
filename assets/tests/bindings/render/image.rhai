// images report their size
let image = Image.new_.call(#{});
assert(image.size.call() == UVec2.new_.call(image.width.call(), image.height.call()), "size");
