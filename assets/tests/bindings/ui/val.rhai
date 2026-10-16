// Val compares by variant only, so the length is checked through the payload, which is the first field
let Node = import_namespace.call("ui").Node;
let Val = import_namespace.call("ui").Val;
let px = Val.px.call(5.0);
assert(px[0] == 5.0, "px payload");
assert(px.variant_name.call() == "Px", "px variant");
let percent = Val.percent.call(50.0);
assert(percent[0] == 50.0 && percent.variant_name.call() == "Percent", "percent");
let vmax = Val.vmax.call(10.0);
assert(vmax[0] == 10.0 && vmax.variant_name.call() == "VMax", "vmax");
let auto = Val.auto.call();
assert(auto.variant_name.call() == "Auto", "auto");

// a length set on a node
let node = Node.new_.call(#{ width: Val.px.call(100.0) });
let width = node.width;
assert(width[0] == 100.0 && width.variant_name.call() == "Px", "node width");
let height = node.height;
assert(height.variant_name.call() == "Auto", "unset fields take their defaults");
