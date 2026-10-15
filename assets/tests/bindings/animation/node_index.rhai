// NodeIndex
let NodeIndex = import_namespace.call("animation").NodeIndex;
let node = NodeIndex.new_.call(3);
assert(node.index.call() == 3, "index returns the number it was made from");
assert(NodeIndex.new_.call(0).index.call() == 0, "index zero");
