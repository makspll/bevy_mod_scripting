// images report their size
let Image = import_namespace.call("render").Image;
let UVec2 = import_namespace.call("math").UVec2;
let image = Image.new_.call(#{});
assert(image.size.call() == UVec2.new_.call(image.width.call(), image.height.call()), "size");
