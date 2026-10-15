-- Val compares by variant only, so the length is checked through the payload, which is the first field
local Node = import_namespace("ui").Node
local Val = import_namespace("ui").Val
assert(Val.px(5)[1] == 5, "px payload")
assert(Val.px(5):variant_name() == "Px", "px variant")
assert(Val.percent(50)[1] == 50 and Val.percent(50):variant_name() == "Percent", "percent")
assert(Val.vw(10)[1] == 10 and Val.vw(10):variant_name() == "Vw", "vw")
assert(Val.vh(10)[1] == 10 and Val.vh(10):variant_name() == "Vh", "vh")
assert(Val.vmin(10)[1] == 10 and Val.vmin(10):variant_name() == "VMin", "vmin")
assert(Val.vmax(10)[1] == 10 and Val.vmax(10):variant_name() == "VMax", "vmax")
assert(Val.auto():variant_name() == "Auto", "auto")

-- a length set on a node
local node = Node.new({ width = Val.px(100) })
assert(node.width[1] == 100 and node.width:variant_name() == "Px", "node width")
assert(node.height:variant_name() == "Auto", "unset fields take their defaults")
