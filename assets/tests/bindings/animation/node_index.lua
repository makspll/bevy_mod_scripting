-- NodeIndex
local NodeIndex = import_namespace("animation").NodeIndex
local node = NodeIndex.new(3)
assert(node:index() == 3, "index returns the number it was made from")
assert(NodeIndex.new(0):index() == 0, "index zero")
